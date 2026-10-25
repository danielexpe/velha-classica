use std::env;
use std::fmt;
use std::io;
use std::process::ExitCode;

use clap::Parser;

use velha_classica::app::App;
use velha_classica::capture;
use velha_classica::cli::{Backend, Cli, ImageProtocolArg};
use velha_classica::theme::{detect_color, ColorEnv, ColorMode};
use velha_classica::util::tty::ensure_tty;

fn main() -> ExitCode {
    let cli = Cli::parse();

    match run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("velha-classica: {error}");
            ExitCode::from(error.code())
        }
    }
}

enum RunError {
    BackendUnavailable(&'static str),
    Capture(String),
    Io(io::Error),
    #[cfg(target_os = "linux")]
    Framebuffer(velha_classica::platform::fb::FbError),
}

impl RunError {
    const fn code(&self) -> u8 {
        match self {
            Self::BackendUnavailable(_) => 3,
            #[cfg(target_os = "linux")]
            Self::Framebuffer(_) => 3,
            Self::Capture(_) | Self::Io(_) => 1,
        }
    }
}

impl fmt::Display for RunError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BackendUnavailable(message) => formatter.write_str(message),
            Self::Capture(message) => write!(formatter, "could not write frame: {message}"),
            Self::Io(error) => write!(formatter, "{error}"),
            #[cfg(target_os = "linux")]
            Self::Framebuffer(error) => write!(formatter, "{error}"),
        }
    }
}

impl From<io::Error> for RunError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

fn run(cli: Cli) -> Result<(), RunError> {
    if let Some(path) = cli.capture_frame.as_deref() {
        capture::write(path, cli.language()).map_err(RunError::Capture)?;
        return Ok(());
    }

    let term = env::var("TERM").ok();
    let colorterm = env::var("COLORTERM").ok();
    let color_env = ColorEnv {
        no_color: env::var_os("NO_COLOR").is_some(),
        colorterm: colorterm.as_deref(),
        term: term.as_deref(),
    };
    let color_mode = detect_color(cli.color.choice(), color_env);
    let mut app = match cli.seed {
        Some(seed) => App::with_seed(cli.language(), color_mode, seed),
        None => App::new(cli.language(), color_mode),
    };
    let images_enabled = !cli.no_images
        && cli.image_protocol.enabled()
        && (color_mode == ColorMode::TrueColor || cli.image_protocol != ImageProtocolArg::Auto);
    app.set_images_enabled(images_enabled);
    app.set_image_protocol(cli.image_protocol.protocol());
    app.set_animations_enabled(!cli.no_animations);
    app.set_audio_enabled(!cli.no_audio);

    if cli.backend == Backend::Fb {
        return run_framebuffer(&mut app, &cli.fb_device);
    }

    ensure_tty()
        .map_err(|_| RunError::BackendUnavailable("stdin and stdout must be a local terminal"))?;
    app.run()?;

    Ok(())
}

#[cfg(target_os = "linux")]
fn run_framebuffer(app: &mut App, device: &std::path::Path) -> Result<(), RunError> {
    app.run_framebuffer(device).map_err(RunError::Framebuffer)
}

#[cfg(not(target_os = "linux"))]
fn run_framebuffer(_app: &mut App, _device: &std::path::Path) -> Result<(), RunError> {
    Err(RunError::BackendUnavailable(
        "the framebuffer backend is only available on Linux",
    ))
}
