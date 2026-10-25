use crate::util::rng::Rng;

pub const SAMPLE_RATE: u32 = 48_000;
pub const DEFAULT_VOLUME: f32 = 0.32;

pub const fn available() -> bool {
    cfg!(all(feature = "audio", target_os = "linux"))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sound {
    Move,
    PlaceX,
    PlaceO,
    Invalid,
    Select,
    Win,
    Draw,
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(not(all(feature = "audio", target_os = "linux")), allow(dead_code))]
enum Command {
    Play(Sound),
    SetEnabled(bool),
    Shutdown,
}

pub struct Audio {
    enabled: bool,
    allow_start: bool,
    volume: f32,
    control: Option<backend::Control>,
}

impl Audio {
    pub fn new(volume: f32) -> Self {
        Self {
            enabled: false,
            allow_start: false,
            volume,
            control: None,
        }
    }

    pub fn allow_start(&mut self) {
        self.allow_start = true;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;

        if enabled && self.allow_start && self.control.is_none() {
            self.control = backend::spawn(self.volume);
        }

        if let Some(control) = self.control.as_ref() {
            control.send(Command::SetEnabled(enabled));
        }
    }

    pub fn play(&self, sound: Sound) {
        if !self.enabled {
            return;
        }

        if let Some(control) = self.control.as_ref() {
            control.send(Command::Play(sound));
        }
    }
}

pub fn synthesize(sound: Sound) -> Vec<f32> {
    match sound {
        Sound::Move => tone(1_200.0, 0.025, 38.0, 0.0),
        Sound::PlaceX => knock(0.080),
        Sound::PlaceO => paper(0.120),
        Sound::Invalid => tone(110.0, 0.070, 28.0, 0.25),
        Sound::Select => tone(660.0, 0.040, 45.0, 0.0),
        Sound::Win => arpeggio(&[523.25, 659.25, 783.99, 1_046.50], 0.15),
        Sound::Draw => arpeggio(&[392.0, 261.63], 0.20),
    }
}

fn tone(frequency: f32, duration: f32, decay: f32, noise: f32) -> Vec<f32> {
    let frames = (duration * SAMPLE_RATE as f32) as usize;
    let mut rng = Rng::new(frames as u64 ^ frequency as u64);
    let mut output = Vec::with_capacity(frames);
    let mut filtered = 0.0;

    for frame in 0..frames {
        let time = frame as f32 / SAMPLE_RATE as f32;
        let envelope = (-decay * time).exp();
        let raw = (std::f32::consts::TAU * frequency * time).sin();
        let hiss = rng.next_u64() as f32 / u64::MAX as f32 * 2.0 - 1.0;
        filtered += 0.25 * (hiss - filtered);
        output.push((raw * (1.0 - noise) + filtered * noise) * envelope * 0.55);
    }

    output
}

fn knock(duration: f32) -> Vec<f32> {
    let frames = (duration * SAMPLE_RATE as f32) as usize;
    let mut rng = Rng::new(0x5EED_1234);
    let mut output = Vec::with_capacity(frames);
    let mut filtered = 0.0;

    for frame in 0..frames {
        let time = frame as f32 / SAMPLE_RATE as f32;
        let envelope = (-42.0 * time).exp();
        let hiss = rng.next_u64() as f32 / u64::MAX as f32 * 2.0 - 1.0;
        filtered += 0.18 * (hiss - filtered);
        let body = (std::f32::consts::TAU * 180.0 * time).sin();
        output.push((filtered * 0.55 + body * 0.45) * envelope * 0.6);
    }

    output
}

fn paper(duration: f32) -> Vec<f32> {
    let frames = (duration * SAMPLE_RATE as f32) as usize;
    let mut rng = Rng::new(0xCAFE_9A11);
    let mut output = Vec::with_capacity(frames);
    let mut filtered = 0.0;

    for frame in 0..frames {
        let time = frame as f32 / SAMPLE_RATE as f32;
        let envelope = (-24.0 * time).exp();
        let hiss = rng.next_u64() as f32 / u64::MAX as f32 * 2.0 - 1.0;
        filtered += 0.06 * (hiss - filtered);
        output.push(filtered * envelope * 0.9);
    }

    output
}

fn arpeggio(notes: &[f32], note_duration: f32) -> Vec<f32> {
    let note_frames = (note_duration * SAMPLE_RATE as f32) as usize;
    let mut output = vec![0.0; note_frames * notes.len()];

    for (index, frequency) in notes.iter().enumerate() {
        let offset = index * note_frames;
        let mut note = tone(*frequency, note_duration, 9.0, 0.0);

        for (frame, sample) in note.drain(..).enumerate() {
            output[offset + frame] += sample;
        }
    }

    output
}

#[cfg(all(feature = "audio", target_os = "linux"))]
mod backend {
    use std::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void, CString};
    use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
    use std::sync::Arc;
    use std::thread::{self, JoinHandle};

    use libloading::Library;

    use super::{synthesize, Command, SAMPLE_RATE};

    const STREAM_PLAYBACK: c_int = 0;
    const FORMAT_FLOAT_LE: c_int = 14;
    const ACCESS_RW_INTERLEAVED: c_int = 3;
    const CHANNELS: c_uint = 1;
    const CHUNK: usize = 512;

    pub struct Control {
        sender: Sender<Command>,
        handle: Option<JoinHandle<()>>,
    }

    impl Control {
        pub fn send(&self, command: Command) {
            let _ = self.sender.send(command);
        }
    }

    impl Drop for Control {
        fn drop(&mut self) {
            let _ = self.sender.send(Command::Shutdown);

            if let Some(handle) = self.handle.take() {
                let _ = handle.join();
            }
        }
    }

    pub fn spawn(volume: f32) -> Option<Control> {
        let device = Device::open()?;
        let (sender, receiver) = mpsc::channel();
        let handle = thread::Builder::new()
            .name("velha-audio".to_string())
            .spawn(move || run(device, receiver, volume))
            .ok()?;

        Some(Control {
            sender,
            handle: Some(handle),
        })
    }

    struct Device {
        alsa: Arc<Alsa>,
        pcm: *mut c_void,
    }

    unsafe impl Send for Device {}

    impl Device {
        fn open() -> Option<Self> {
            let alsa = Arc::new(Alsa::load()?);
            let name = CString::new("default").ok()?;
            let mut pcm: *mut c_void = std::ptr::null_mut();

            let result = unsafe { (alsa.open)(&mut pcm, name.as_ptr(), STREAM_PLAYBACK, 0) };

            if result < 0 || pcm.is_null() {
                return None;
            }

            let result = unsafe {
                (alsa.set_params)(
                    pcm,
                    FORMAT_FLOAT_LE,
                    ACCESS_RW_INTERLEAVED,
                    CHANNELS,
                    SAMPLE_RATE,
                    1,
                    60_000,
                )
            };

            if result < 0 {
                unsafe {
                    (alsa.close)(pcm);
                }
                return None;
            }

            Some(Self { alsa, pcm })
        }

        fn write(&self, samples: &[f32]) {
            if samples.is_empty() {
                return;
            }

            let result = unsafe {
                (self.alsa.writei)(
                    self.pcm,
                    samples.as_ptr() as *const c_void,
                    samples.len() as c_ulong,
                )
            };

            if result < 0 {
                unsafe {
                    (self.alsa.recover)(self.pcm, result as c_int, 1);
                }
            }
        }
    }

    impl Drop for Device {
        fn drop(&mut self) {
            unsafe {
                (self.alsa.drain)(self.pcm);
                (self.alsa.close)(self.pcm);
            }
        }
    }

    struct Alsa {
        _library: Library,
        open: unsafe extern "C" fn(*mut *mut c_void, *const c_char, c_int, c_int) -> c_int,
        close: unsafe extern "C" fn(*mut c_void) -> c_int,
        set_params:
            unsafe extern "C" fn(*mut c_void, c_int, c_int, c_uint, c_uint, c_int, c_uint) -> c_int,
        writei: unsafe extern "C" fn(*mut c_void, *const c_void, c_ulong) -> c_long,
        drain: unsafe extern "C" fn(*mut c_void) -> c_int,
        recover: unsafe extern "C" fn(*mut c_void, c_int, c_int) -> c_int,
    }

    impl Alsa {
        fn load() -> Option<Self> {
            let library = unsafe { Library::new("libasound.so.2") }.ok()?;

            unsafe {
                let open = *library.get(b"snd_pcm_open\0").ok()?;
                let close = *library.get(b"snd_pcm_close\0").ok()?;
                let set_params = *library.get(b"snd_pcm_set_params\0").ok()?;
                let writei = *library.get(b"snd_pcm_writei\0").ok()?;
                let drain = *library.get(b"snd_pcm_drain\0").ok()?;
                let recover = *library.get(b"snd_pcm_recover\0").ok()?;

                Some(Self {
                    _library: library,
                    open,
                    close,
                    set_params,
                    writei,
                    drain,
                    recover,
                })
            }
        }
    }

    struct Playing {
        samples: Vec<f32>,
        cursor: usize,
    }

    fn run(device: Device, receiver: Receiver<Command>, volume: f32) {
        let mut enabled = true;
        let mut playing: Vec<Playing> = Vec::new();

        loop {
            while playing.is_empty() {
                match receiver.recv() {
                    Ok(command) => {
                        if !apply(command, &mut enabled, &mut playing) {
                            return;
                        }
                    }
                    Err(_) => return,
                }
            }

            loop {
                match receiver.try_recv() {
                    Ok(command) => {
                        if !apply(command, &mut enabled, &mut playing) {
                            return;
                        }
                    }
                    Err(TryRecvError::Empty) => break,
                    Err(TryRecvError::Disconnected) => return,
                }
            }

            if playing.is_empty() {
                continue;
            }

            let mut buffer = vec![0.0f32; CHUNK];

            for sample in &mut buffer {
                let mut mixed = 0.0;

                for sound in &mut playing {
                    if let Some(value) = sound.samples.get(sound.cursor) {
                        mixed += value;
                        sound.cursor += 1;
                    }
                }

                *sample = (mixed * volume).clamp(-1.0, 1.0);
            }

            playing.retain(|sound| sound.cursor < sound.samples.len());
            device.write(&buffer);
        }
    }

    fn apply(command: Command, enabled: &mut bool, playing: &mut Vec<Playing>) -> bool {
        match command {
            Command::Play(sound) => {
                if *enabled {
                    playing.push(Playing {
                        samples: synthesize(sound),
                        cursor: 0,
                    });
                }
            }
            Command::SetEnabled(value) => {
                *enabled = value;

                if !value {
                    playing.clear();
                }
            }
            Command::Shutdown => return false,
        }

        true
    }
}

#[cfg(not(all(feature = "audio", target_os = "linux")))]
mod backend {
    use super::Command;

    pub struct Control;

    impl Control {
        pub fn send(&self, _command: Command) {}
    }

    pub fn spawn(_volume: f32) -> Option<Control> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn peak(samples: &[f32]) -> f32 {
        samples
            .iter()
            .fold(0.0f32, |peak, sample| peak.max(sample.abs()))
    }

    #[test]
    fn every_sound_has_samples_in_range() {
        for sound in [
            Sound::Move,
            Sound::PlaceX,
            Sound::PlaceO,
            Sound::Invalid,
            Sound::Select,
            Sound::Win,
            Sound::Draw,
        ] {
            let samples = synthesize(sound);

            assert!(!samples.is_empty());
            assert!(samples.iter().all(|sample| sample.is_finite()));
            assert!(peak(&samples) <= 1.0);
        }
    }

    #[test]
    fn effects_have_the_expected_durations() {
        let frames = |seconds: f32| (seconds * SAMPLE_RATE as f32) as usize;

        assert_eq!(synthesize(Sound::Move).len(), frames(0.025));
        assert_eq!(synthesize(Sound::PlaceX).len(), frames(0.080));
        assert_eq!(synthesize(Sound::PlaceO).len(), frames(0.120));
        assert_eq!(synthesize(Sound::Win).len(), frames(0.15) * 4);
        assert_eq!(synthesize(Sound::Draw).len(), frames(0.20) * 2);
    }

    #[test]
    fn synthesis_is_deterministic() {
        for sound in [Sound::PlaceX, Sound::PlaceO, Sound::Win] {
            assert_eq!(synthesize(sound), synthesize(sound));
        }
    }

    #[test]
    fn audio_without_start_is_silent_and_safe() {
        let mut audio = Audio::new(DEFAULT_VOLUME);

        audio.set_enabled(true);
        audio.play(Sound::Win);
        audio.set_enabled(false);
    }
}
