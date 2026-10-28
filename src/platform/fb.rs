use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::os::fd::{AsRawFd, RawFd};
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub const DEFAULT_DEVICE: &str = "/dev/fb0";

const TTY_PATH: &str = "/dev/tty";

const FBIOGET_VSCREENINFO: libc::c_ulong = 0x4600;
const FBIOGET_FSCREENINFO: libc::c_ulong = 0x4602;
const FBIOPAN_DISPLAY: libc::c_ulong = 0x4606;

const KDGETMODE: libc::c_ulong = 0x4B3B;
const KDSETMODE: libc::c_ulong = 0x4B3A;
const KD_TEXT: libc::c_int = 0x00;
const KD_GRAPHICS: libc::c_int = 0x01;

const CURSOR_HIDE: &[u8] = b"\x1b[?25l\x1b[2J\x1b[H";
const CURSOR_SHOW: &[u8] = b"\x1b[0m\x1b[?25h\x1b[2J\x1b[H";

#[derive(Debug, thiserror::Error)]
pub enum FbError {
    #[error("{path}: {source}")]
    Io {
        path: String,
        #[source]
        source: io::Error,
    },
    #[error("unsupported framebuffer format: {0}")]
    Unsupported(String),
    #[error("the framebuffer backend needs a local virtual console: {0}")]
    NotConsole(String),
    #[error("could not install signal handlers: {0}")]
    Signal(#[source] io::Error),
}

impl FbError {
    fn io(path: &Path, source: io::Error) -> Self {
        let path = if source.kind() == io::ErrorKind::PermissionDenied {
            format!(
                "{} (add the user to the video group and log in again)",
                path.display()
            )
        } else {
            path.display().to_string()
        };

        Self::Io { path, source }
    }

    fn io_owned(path: &str, source: io::Error) -> Self {
        Self::io(Path::new(path), source)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
struct FbBitfield {
    offset: u32,
    length: u32,
    msb_right: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
struct FbVarScreenInfo {
    xres: u32,
    yres: u32,
    xres_virtual: u32,
    yres_virtual: u32,
    xoffset: u32,
    yoffset: u32,
    bits_per_pixel: u32,
    grayscale: u32,
    red: FbBitfield,
    green: FbBitfield,
    blue: FbBitfield,
    transp: FbBitfield,
    nonstd: u32,
    activate: u32,
    height: u32,
    width: u32,
    accel_flags: u32,
    pixclock: u32,
    left_margin: u32,
    right_margin: u32,
    upper_margin: u32,
    lower_margin: u32,
    hsync_len: u32,
    vsync_len: u32,
    sync: u32,
    vmode: u32,
    rotate: u32,
    colorspace: u32,
    reserved: [u32; 4],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
struct FbFixScreenInfo {
    id: [u8; 16],
    smem_start: libc::c_ulong,
    smem_len: u32,
    fb_type: u32,
    type_aux: u32,
    visual: u32,
    xpanstep: u16,
    ypanstep: u16,
    ywrapstep: u16,
    line_length: u32,
    mmio_start: libc::c_ulong,
    mmio_len: u32,
    accel: u32,
    capabilities: u16,
    reserved: [u16; 2],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Channel {
    pub offset: u32,
    pub length: u32,
}

impl Channel {
    const fn new(offset: u32, length: u32) -> Self {
        Self { offset, length }
    }

    fn scaled(self, value: u8) -> u32 {
        if self.length == 0 {
            return 0;
        }

        let max = (1u32 << self.length) - 1;
        let scaled = (u32::from(value) * max + 127) / 255;

        (scaled & max) << self.offset
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PixelFormat {
    pub bits_per_pixel: u32,
    pub red: Channel,
    pub green: Channel,
    pub blue: Channel,
    pub alpha: Channel,
}

impl PixelFormat {
    pub fn parse(
        bits_per_pixel: u32,
        red: Channel,
        green: Channel,
        blue: Channel,
        alpha: Channel,
    ) -> Result<Self, FbError> {
        let rgb_888 = red.length == 8 && green.length == 8 && blue.length == 8;

        let supported = match bits_per_pixel {
            32 => rgb_888 && (alpha.length == 0 || alpha.length == 8),
            24 => rgb_888 && alpha.length == 0,
            16 => red.length == 5 && green.length == 6 && blue.length == 5 && alpha.length == 0,
            _ => false,
        };

        if !supported {
            return Err(FbError::Unsupported(format!(
                "{bits_per_pixel} bpp R{}@{} G{}@{} B{}@{} A{}@{}",
                red.length,
                red.offset,
                green.length,
                green.offset,
                blue.length,
                blue.offset,
                alpha.length,
                alpha.offset
            )));
        }

        Ok(Self {
            bits_per_pixel,
            red,
            green,
            blue,
            alpha,
        })
    }

    pub const fn bytes_per_pixel(self) -> usize {
        (self.bits_per_pixel / 8) as usize
    }

    pub fn pack(self, red: u8, green: u8, blue: u8, alpha: u8) -> u32 {
        self.red.scaled(red)
            | self.green.scaled(green)
            | self.blue.scaled(blue)
            | self.alpha.scaled(alpha)
    }

    pub fn write_row(self, rgba: &[u8], out: &mut [u8]) {
        let bytes = self.bytes_per_pixel();

        for (pixel, chunk) in rgba
            .as_chunks::<4>()
            .0
            .iter()
            .zip(out.chunks_exact_mut(bytes))
        {
            let value = self.pack(pixel[0], pixel[1], pixel[2], pixel[3]);
            let encoded = value.to_le_bytes();

            chunk.copy_from_slice(&encoded[..bytes]);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceInfo {
    pub width: u32,
    pub height: u32,
    pub line_length: usize,
    pub format: PixelFormat,
    pub pan_buffers: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MappingPlan {
    pan: bool,
    yoffset: u32,
    mapping_len: usize,
}

fn plan_mapping(
    yres: u32,
    yres_virtual: u32,
    yoffset: u32,
    ypanstep: u16,
    line_length: usize,
    row_bytes: usize,
    smem_len: usize,
) -> Result<MappingPlan, FbError> {
    if line_length < row_bytes {
        return Err(FbError::Unsupported(format!(
            "line length {line_length} is smaller than {row_bytes} bytes per row"
        )));
    }

    let max_yoffset = yres_virtual.checked_sub(yres).ok_or_else(|| {
        FbError::Unsupported(format!(
            "virtual height {yres_virtual} is smaller than the visible height {yres}"
        ))
    })?;

    if yoffset > max_yoffset {
        return Err(FbError::Unsupported(format!(
            "yoffset {yoffset} exceeds the usable virtual range (0..={max_yoffset})"
        )));
    }

    let visible_rows = (yoffset as usize)
        .checked_add(yres as usize)
        .ok_or_else(|| FbError::Unsupported("framebuffer size overflow".into()))?;
    let visible_len = line_length
        .checked_mul(visible_rows)
        .ok_or_else(|| FbError::Unsupported("framebuffer size overflow".into()))?;
    let double_rows = (yres as usize)
        .checked_mul(2)
        .ok_or_else(|| FbError::Unsupported("framebuffer size overflow".into()))?;
    let double_len = line_length
        .checked_mul(double_rows)
        .ok_or_else(|| FbError::Unsupported("framebuffer size overflow".into()))?;

    let pan_candidate = ypanstep > 0
        && yres_virtual >= yres.saturating_mul(2)
        && yres.is_multiple_of(u32::from(ypanstep));
    let pan = pan_candidate && smem_len >= double_len;
    let mapping_len = if pan {
        double_len.max(visible_len)
    } else {
        visible_len
    };

    if mapping_len == 0 {
        return Err(FbError::Unsupported(
            "framebuffer reports zero bytes of visible memory".into(),
        ));
    }

    if smem_len < mapping_len {
        return Err(FbError::Unsupported(format!(
            "framebuffer memory has {smem_len} bytes, but {mapping_len} are required"
        )));
    }

    Ok(MappingPlan {
        pan,
        yoffset,
        mapping_len,
    })
}

fn next_buffer(current: u32, yres: u32) -> u32 {
    if current == 0 {
        yres
    } else {
        0
    }
}

pub struct Device {
    file: File,
    var: FbVarScreenInfo,
    info: DeviceInfo,
    mapping: *mut u8,
    mapping_len: usize,
    current_yoffset: u32,
    pan_enabled: bool,
}

impl Device {
    pub fn open(path: &Path) -> Result<Self, FbError> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(libc::O_CLOEXEC)
            .open(path)
            .map_err(|error| FbError::io(path, error))?;
        let fd = file.as_raw_fd();
        let var: FbVarScreenInfo = get_ioctl(fd, FBIOGET_VSCREENINFO, path)?;
        let fix: FbFixScreenInfo = get_ioctl(fd, FBIOGET_FSCREENINFO, path)?;

        if var.xres == 0 || var.yres == 0 {
            return Err(FbError::Unsupported(
                "the framebuffer reports a zero size".into(),
            ));
        }

        let format = PixelFormat::parse(
            var.bits_per_pixel,
            Channel::new(var.red.offset, var.red.length),
            Channel::new(var.green.offset, var.green.length),
            Channel::new(var.blue.offset, var.blue.length),
            Channel::new(var.transp.offset, var.transp.length),
        )?;

        let line_length = fix.line_length as usize;
        let row_bytes = var.xres as usize * format.bytes_per_pixel();
        let plan = plan_mapping(
            var.yres,
            var.yres_virtual,
            var.yoffset,
            fix.ypanstep,
            line_length,
            row_bytes,
            fix.smem_len as usize,
        )?;
        let mapping_len = plan.mapping_len;

        let mapping = unsafe {
            // SAFETY: the descriptor is open read/write and the length comes from the driver.
            libc::mmap(
                ptr::null_mut(),
                mapping_len,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED,
                fd,
                0,
            )
        };

        if mapping == libc::MAP_FAILED {
            return Err(FbError::io(path, io::Error::last_os_error()));
        }

        Ok(Self {
            file,
            var,
            info: DeviceInfo {
                width: var.xres,
                height: var.yres,
                line_length,
                format,
                pan_buffers: if plan.pan { 2 } else { 1 },
            },
            mapping: mapping.cast(),
            mapping_len,
            current_yoffset: plan.yoffset,
            pan_enabled: plan.pan,
        })
    }

    pub const fn info(&self) -> DeviceInfo {
        self.info
    }

    pub fn present(&mut self, rgba: &[u8]) -> Result<(), FbError> {
        let DeviceInfo { width, height, .. } = self.info;
        let expected = width as usize * height as usize * 4;

        if rgba.len() != expected {
            return Err(FbError::Unsupported(format!(
                "frame has {} bytes, expected {expected}",
                rgba.len()
            )));
        }

        if self.pan_enabled {
            let target = next_buffer(self.current_yoffset, height);
            self.write_frame(rgba, target)?;

            if self.pan_to(target).is_ok() {
                self.current_yoffset = target;
                return Ok(());
            }

            self.pan_enabled = false;
            self.info.pan_buffers = 1;
            self.var.yoffset = self.current_yoffset;
        }

        self.write_frame(rgba, self.current_yoffset)
    }

    fn write_frame(&mut self, rgba: &[u8], target_y: u32) -> Result<(), FbError> {
        let DeviceInfo {
            width,
            height,
            line_length,
            format,
            ..
        } = self.info;
        let row_bytes = width as usize * format.bytes_per_pixel();
        let offset = (target_y as usize)
            .checked_mul(line_length)
            .ok_or_else(|| FbError::Unsupported("framebuffer offset overflow".into()))?;
        let frame_len = (height as usize)
            .checked_mul(line_length)
            .and_then(|len| offset.checked_add(len))
            .ok_or_else(|| FbError::Unsupported("framebuffer size overflow".into()))?;

        if frame_len > self.mapping_len {
            return Err(FbError::Unsupported(
                "the frame does not fit in the framebuffer memory".into(),
            ));
        }

        let destination = unsafe {
            // SAFETY: the offset and frame length were bounds-checked against the mapping above.
            std::slice::from_raw_parts_mut(self.mapping.add(offset), height as usize * line_length)
        };

        for (row, source) in rgba.chunks_exact(width as usize * 4).enumerate() {
            let start = row * line_length;
            let target = &mut destination[start..start + row_bytes];
            format.write_row(source, target);
        }

        Ok(())
    }

    fn pan_to(&mut self, target_y: u32) -> Result<(), FbError> {
        self.var.yoffset = target_y;
        let result = unsafe {
            // SAFETY: the descriptor is a valid framebuffer and the struct matches the ABI.
            libc::ioctl(self.file.as_raw_fd(), FBIOPAN_DISPLAY, &self.var)
        };

        if result == -1 {
            return Err(FbError::io_owned(
                "FBIOPAN_DISPLAY",
                io::Error::last_os_error(),
            ));
        }

        Ok(())
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        if !self.mapping.is_null() {
            unsafe {
                // SAFETY: the pointer and length are the ones returned by mmap.
                libc::munmap(self.mapping.cast(), self.mapping_len);
            }
            self.mapping = ptr::null_mut();
        }
    }
}

pub struct Console {
    file: File,
    previous_mode: libc::c_int,
    active: bool,
}

impl Console {
    pub fn enter() -> Result<Self, FbError> {
        let path = Path::new(TTY_PATH);
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(libc::O_CLOEXEC)
            .open(path)
            .map_err(|error| FbError::io(path, error))?;
        let fd = file.as_raw_fd();
        let previous_mode = get_ioctl::<libc::c_int>(fd, KDGETMODE, path).map_err(|error| {
            FbError::NotConsole(format!("{error}; run the game from a local VT console"))
        })?;
        let result = unsafe {
            // SAFETY: the descriptor refers to /dev/tty and KDSETMODE takes an int argument.
            libc::ioctl(fd, KDSETMODE, KD_GRAPHICS)
        };

        if result == -1 {
            let error = io::Error::last_os_error();

            return Err(FbError::NotConsole(format!(
                "cannot switch /dev/tty to graphics mode: {error}; run the game from a local VT console"
            )));
        }

        let mut console = Self {
            file,
            previous_mode,
            active: true,
        };

        console.write(CURSOR_HIDE)?;

        Ok(console)
    }

    fn write(&mut self, bytes: &[u8]) -> Result<(), FbError> {
        self.file
            .write_all(bytes)
            .map_err(|error| FbError::io_owned(TTY_PATH, error))
    }

    pub fn restore(&mut self) -> Result<(), FbError> {
        if !self.active {
            return Ok(());
        }

        self.active = false;
        let painted = self.write(CURSOR_SHOW);
        let result = unsafe {
            // SAFETY: the descriptor refers to /dev/tty and KDSETMODE takes an int argument.
            libc::ioctl(self.file.as_raw_fd(), KDSETMODE, self.previous_mode)
        };

        if result == -1 {
            return Err(FbError::io_owned(TTY_PATH, io::Error::last_os_error()));
        }

        painted
    }
}

impl Drop for Console {
    fn drop(&mut self) {
        let _ = self.restore();
    }
}

pub struct Session {
    device: Device,
    console: Console,
    raw: bool,
}

impl Session {
    pub fn open(device: &Path) -> Result<Self, FbError> {
        let device = Device::open(device)?;
        let console = Console::enter()?;

        if let Err(error) = enable_raw_mode() {
            drop(console);
            return Err(FbError::io_owned(TTY_PATH, error));
        }

        Ok(Self {
            device,
            console,
            raw: true,
        })
    }

    pub const fn info(&self) -> DeviceInfo {
        self.device.info()
    }

    pub fn present(&mut self, rgba: &[u8]) -> Result<(), FbError> {
        self.device.present(rgba)
    }

    pub fn restore(&mut self) -> Result<(), FbError> {
        if self.raw {
            self.raw = false;
            let _ = disable_raw_mode();
        }

        self.console.restore()
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        let _ = self.restore();
    }
}

fn get_ioctl<T>(fd: RawFd, request: libc::c_ulong, path: &Path) -> Result<T, FbError> {
    let mut value = std::mem::MaybeUninit::<T>::zeroed();
    let result = unsafe {
        // SAFETY: the buffer is large enough for the struct requested by the ioctl.
        libc::ioctl(fd, request, value.as_mut_ptr())
    };

    if result == -1 {
        return Err(FbError::io(path, io::Error::last_os_error()));
    }

    Ok(unsafe {
        // SAFETY: the ioctl succeeded and initialized the whole struct.
        value.assume_init()
    })
}

static TERMINATE: AtomicBool = AtomicBool::new(false);

extern "C" fn on_terminate(_signal: libc::c_int) {
    TERMINATE.store(true, Ordering::Relaxed);
}

pub fn install_signal_handlers() -> io::Result<()> {
    TERMINATE.store(false, Ordering::Relaxed);

    for signal in [libc::SIGTERM, libc::SIGINT, libc::SIGHUP] {
        let mut action: libc::sigaction = unsafe {
            // SAFETY: a zeroed sigaction is valid before the fields are filled in.
            std::mem::zeroed()
        };
        action.sa_sigaction = on_terminate as *const () as libc::sighandler_t;
        action.sa_flags = libc::SA_RESTART;

        unsafe {
            // SAFETY: the handler is a valid extern "C" function and the mask is initialized.
            libc::sigemptyset(&mut action.sa_mask);
        }

        let result = unsafe {
            // SAFETY: `action` outlives the call and no old handler is requested.
            libc::sigaction(signal, &action, ptr::null_mut())
        };

        if result == -1 {
            return Err(io::Error::last_os_error());
        }
    }

    Ok(())
}

pub fn terminate_requested() -> bool {
    TERMINATE.load(Ordering::Relaxed)
}

pub fn install_panic_hook() {
    let previous = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |info| {
        emergency_restore();
        previous(info);
    }));
}

pub fn emergency_restore() {
    let _ = disable_raw_mode();

    if let Ok(mut file) = OpenOptions::new().write(true).open(TTY_PATH) {
        let _ = file.write_all(CURSOR_SHOW);
        unsafe {
            // SAFETY: the descriptor refers to /dev/tty and KDSETMODE takes an int argument.
            libc::ioctl(file.as_raw_fd(), KDSETMODE, KD_TEXT);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn format(
        bpp: u32,
        red: (u32, u32),
        green: (u32, u32),
        blue: (u32, u32),
        alpha: (u32, u32),
    ) -> PixelFormat {
        PixelFormat::parse(
            bpp,
            Channel::new(red.0, red.1),
            Channel::new(green.0, green.1),
            Channel::new(blue.0, blue.1),
            Channel::new(alpha.0, alpha.1),
        )
        .unwrap()
    }

    #[test]
    fn kernel_struct_sizes_match_the_abi() {
        assert_eq!(std::mem::size_of::<FbVarScreenInfo>(), 160);
        assert_eq!(std::mem::size_of::<FbFixScreenInfo>(), 80);
    }

    #[test]
    fn accepts_rgbx_and_bgrx_32bpp() {
        let rgbx = format(32, (16, 8), (8, 8), (0, 8), (24, 8));
        let bgrx = format(32, (0, 8), (8, 8), (16, 8), (24, 0));

        assert_eq!(rgbx.pack(0x12, 0x34, 0x56, 0xff), 0xFF12_3456);
        assert_eq!(bgrx.pack(0x12, 0x34, 0x56, 0xff), 0x0056_3412);
        assert_eq!(rgbx.bytes_per_pixel(), 4);
    }

    #[test]
    fn accepts_24bpp_and_16bpp() {
        let rgb = format(24, (16, 8), (8, 8), (0, 8), (0, 0));
        let rgb565 = format(16, (11, 5), (5, 6), (0, 5), (0, 0));

        assert_eq!(rgb.pack(0x12, 0x34, 0x56, 0xff), 0x12_3456);
        assert_eq!(rgb565.pack(0xff, 0xff, 0xff, 0xff), 0xffff);
        assert_eq!(rgb565.pack(0, 0, 0, 0), 0);
        assert_eq!(rgb565.bytes_per_pixel(), 2);
    }

    #[test]
    fn rejects_unsupported_layouts() {
        assert!(PixelFormat::parse(
            8,
            Channel::new(0, 8),
            Channel::new(0, 8),
            Channel::new(0, 8),
            Channel::new(0, 0)
        )
        .is_err());
        assert!(PixelFormat::parse(
            32,
            Channel::new(0, 8),
            Channel::new(8, 8),
            Channel::new(16, 8),
            Channel::new(24, 4)
        )
        .is_err());
        assert!(PixelFormat::parse(
            16,
            Channel::new(0, 5),
            Channel::new(5, 5),
            Channel::new(10, 5),
            Channel::new(0, 0)
        )
        .is_err());
    }

    #[test]
    fn writes_rows_in_the_native_byte_order() {
        let rgb565 = format(16, (11, 5), (5, 6), (0, 5), (0, 0));
        let rgba = [0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0xff];
        let mut out = [0u8; 4];

        rgb565.write_row(&rgba, &mut out);

        assert_eq!(out, [0xff, 0xff, 0x00, 0x00]);
    }

    fn plan(
        yres: u32,
        yres_virtual: u32,
        yoffset: u32,
        ypanstep: u16,
        line_length: usize,
        smem_len: usize,
    ) -> Result<MappingPlan, FbError> {
        plan_mapping(
            yres,
            yres_virtual,
            yoffset,
            ypanstep,
            line_length,
            4,
            smem_len,
        )
    }

    #[test]
    fn chooses_the_buffer_that_is_not_visible() {
        assert_eq!(next_buffer(0, 1080), 1080);
        assert_eq!(next_buffer(1080, 1080), 0);
        assert_eq!(next_buffer(42, 1080), 0);
    }

    #[test]
    fn plans_double_buffer_when_ypan_and_memory_allow() {
        let line_length = 7680;
        let plan = plan(1080, 2160, 0, 1, line_length, line_length * 2160).unwrap();

        assert!(plan.pan);
        assert_eq!(plan.yoffset, 0);
        assert_eq!(plan.mapping_len, line_length * 2160);
    }

    #[test]
    fn keeps_the_current_yoffset_when_pan_is_unavailable() {
        let line_length = 7680;
        let plan = plan(1080, 2160, 1080, 0, line_length, line_length * 2160).unwrap();

        assert!(!plan.pan);
        assert_eq!(plan.yoffset, 1080);
        assert_eq!(plan.mapping_len, line_length * 2160);
    }

    #[test]
    fn disables_pan_when_memory_only_covers_one_buffer() {
        let line_length = 7680;
        let plan = plan(1080, 2160, 0, 1, line_length, line_length * 1080).unwrap();

        assert!(!plan.pan);
        assert_eq!(plan.mapping_len, line_length * 1080);
    }

    #[test]
    fn maps_the_visible_buffer_for_pan_fallback() {
        let line_length = 400;
        let plan = plan(100, 300, 200, 1, line_length, line_length * 300).unwrap();

        assert!(plan.pan);
        assert_eq!(plan.yoffset, 200);
        assert_eq!(plan.mapping_len, line_length * 300);
    }

    #[test]
    fn rejects_invalid_yoffset_and_stride() {
        assert!(plan(1080, 2160, 1081, 1, 7680, 7680 * 2160).is_err());
        assert!(plan(1080, 1000, 0, 1, 7680, 7680 * 2160).is_err());
        assert!(plan_mapping(1080, 2160, 0, 1, 4000, 4320, 7680 * 2160).is_err());
    }

    #[test]
    fn rejects_insufficient_framebuffer_memory() {
        let line_length = 7680;

        assert!(plan(1080, 1080, 0, 1, line_length, line_length * 1080 - 1).is_err());
    }
}
