use clap::{Parser, ValueEnum};

use std::path::PathBuf;

use crate::i18n::Language;
use crate::render::image::ImageProtocol;
use crate::theme::ColorChoice;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Default)]
pub enum LanguageArg {
    #[default]
    Pt,
    En,
}

impl LanguageArg {
    pub const fn language(self) -> Language {
        match self {
            Self::Pt => Language::PtBr,
            Self::En => Language::En,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Default)]
pub enum Backend {
    #[default]
    Auto,
    Tui,
    Fb,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Default)]
pub enum ColorArg {
    #[default]
    Auto,
    Truecolor,
    #[value(name = "256")]
    Ansi256,
    #[value(name = "16")]
    Ansi16,
    Never,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Default)]
pub enum ImageProtocolArg {
    #[default]
    Auto,
    Kitty,
    Sixel,
    Iterm2,
    Halfblocks,
    None,
}

impl ImageProtocolArg {
    pub const fn protocol(self) -> ImageProtocol {
        match self {
            Self::Auto => ImageProtocol::Auto,
            Self::Kitty => ImageProtocol::Kitty,
            Self::Sixel => ImageProtocol::Sixel,
            Self::Iterm2 => ImageProtocol::Iterm2,
            Self::Halfblocks => ImageProtocol::Halfblocks,
            Self::None => ImageProtocol::Halfblocks,
        }
    }

    pub const fn enabled(self) -> bool {
        !matches!(self, Self::None)
    }
}

impl ColorArg {
    pub const fn choice(self) -> ColorChoice {
        match self {
            Self::Auto => ColorChoice::Auto,
            Self::Truecolor => ColorChoice::TrueColor,
            Self::Ansi256 => ColorChoice::Ansi256,
            Self::Ansi16 => ColorChoice::Ansi16,
            Self::Never => ColorChoice::Never,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[command(
    name = "velha-classica",
    version,
    about = "Classic tic-tac-toe for the Linux terminal",
    long_about = None
)]
pub struct Cli {
    #[arg(
        long,
        value_enum,
        value_name = "pt|en",
        default_value = "pt",
        help = "Initial UI language"
    )]
    pub lang: LanguageArg,

    #[arg(
        long,
        value_enum,
        value_name = "auto|tui|fb",
        default_value = "auto",
        help = "Video backend"
    )]
    pub backend: Backend,

    #[arg(
        long,
        value_name = "PATH",
        default_value = "/dev/fb0",
        help = "Framebuffer device used by the fb backend"
    )]
    pub fb_device: PathBuf,

    #[arg(long, value_name = "N", help = "Seed for reproducible AI moves")]
    pub seed: Option<u64>,

    #[arg(long, help = "Disable PNG board rendering")]
    pub no_images: bool,

    #[arg(
        long,
        value_enum,
        value_name = "auto|kitty|sixel|iterm2|halfblocks|none",
        default_value = "auto",
        help = "Terminal image protocol"
    )]
    pub image_protocol: ImageProtocolArg,

    #[arg(long, help = "Reduce motion and AI thinking delay")]
    pub no_animations: bool,

    #[arg(long, help = "Disable sound effects")]
    pub no_audio: bool,

    #[arg(
        long,
        value_enum,
        value_name = "auto|truecolor|256|16|never",
        default_value = "auto",
        help = "Color mode"
    )]
    pub color: ColorArg,

    #[arg(
        long,
        value_name = "PATH",
        help = "Write a deterministic QA frame and exit"
    )]
    pub capture_frame: Option<PathBuf>,
}

impl Cli {
    pub const fn language(&self) -> Language {
        self.lang.language()
    }
}

#[cfg(test)]
mod tests {
    use clap::error::ErrorKind;
    use clap::CommandFactory;

    use super::*;

    #[test]
    fn defaults_are_pt_br_and_auto() {
        let cli = Cli::try_parse_from(["velha-classica"]).unwrap();

        assert_eq!(cli.language(), Language::PtBr);
        assert_eq!(cli.backend, Backend::Auto);
        assert_eq!(cli.seed, None);
        assert_eq!(cli.color, ColorArg::Auto);
        assert!(!cli.no_images);
        assert!(!cli.no_animations);
        assert!(!cli.no_audio);
        assert_eq!(cli.image_protocol, ImageProtocolArg::Auto);
        assert_eq!(cli.capture_frame, None);
        assert_eq!(cli.fb_device, PathBuf::from("/dev/fb0"));
    }

    #[test]
    fn parses_language_and_backend() {
        let cli =
            Cli::try_parse_from(["velha-classica", "--lang", "en", "--backend", "fb"]).unwrap();

        assert_eq!(cli.language(), Language::En);
        assert_eq!(cli.backend, Backend::Fb);
        assert_eq!(cli.seed, None);
    }

    #[test]
    fn parses_framebuffer_device() {
        let cli = Cli::try_parse_from([
            "velha-classica",
            "--backend",
            "fb",
            "--fb-device",
            "/dev/fb1",
        ])
        .unwrap();

        assert_eq!(cli.fb_device, PathBuf::from("/dev/fb1"));
    }

    #[test]
    fn parses_feature_flags_and_color() {
        let cli = Cli::try_parse_from([
            "velha-classica",
            "--no-images",
            "--no-animations",
            "--no-audio",
            "--color",
            "256",
            "--image-protocol",
            "sixel",
        ])
        .unwrap();

        assert!(cli.no_images);
        assert!(cli.no_animations);
        assert!(cli.no_audio);
        assert_eq!(cli.color, ColorArg::Ansi256);
        assert_eq!(cli.image_protocol, ImageProtocolArg::Sixel);
    }

    #[test]
    fn image_protocol_none_disables_images() {
        let cli = Cli::try_parse_from(["velha-classica", "--image-protocol", "none"]).unwrap();

        assert!(!cli.image_protocol.enabled());
    }

    #[test]
    fn image_protocol_argument_maps_to_protocol() {
        assert_eq!(ImageProtocolArg::Auto.protocol(), ImageProtocol::Auto);
        assert_eq!(ImageProtocolArg::Kitty.protocol(), ImageProtocol::Kitty);
        assert_eq!(ImageProtocolArg::Sixel.protocol(), ImageProtocol::Sixel);
        assert_eq!(ImageProtocolArg::Iterm2.protocol(), ImageProtocol::Iterm2);
        assert_eq!(
            ImageProtocolArg::Halfblocks.protocol(),
            ImageProtocol::Halfblocks
        );
    }

    #[test]
    fn color_argument_maps_to_color_choice() {
        assert_eq!(ColorArg::Auto.choice(), ColorChoice::Auto);
        assert_eq!(ColorArg::Truecolor.choice(), ColorChoice::TrueColor);
        assert_eq!(ColorArg::Ansi256.choice(), ColorChoice::Ansi256);
        assert_eq!(ColorArg::Ansi16.choice(), ColorChoice::Ansi16);
        assert_eq!(ColorArg::Never.choice(), ColorChoice::Never);
    }

    #[test]
    fn parses_capture_frame() {
        let cli =
            Cli::try_parse_from(["velha-classica", "--capture-frame", "/tmp/frame.png"]).unwrap();

        assert_eq!(cli.capture_frame, Some(PathBuf::from("/tmp/frame.png")));
    }

    #[test]
    fn parses_seed() {
        let cli = Cli::try_parse_from(["velha-classica", "--seed", "42"]).unwrap();

        assert_eq!(cli.seed, Some(42));
    }

    #[test]
    fn rejects_unknown_language() {
        let error = Cli::try_parse_from(["velha-classica", "--lang", "fr"]).unwrap_err();

        assert_eq!(error.kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn help_and_version_are_available() {
        let help = Cli::try_parse_from(["velha-classica", "--help"]).unwrap_err();
        let version = Cli::try_parse_from(["velha-classica", "--version"]).unwrap_err();

        assert_eq!(help.kind(), ErrorKind::DisplayHelp);
        assert_eq!(version.kind(), ErrorKind::DisplayVersion);
    }

    #[test]
    fn command_definition_is_valid() {
        Cli::command().debug_assert();
    }
}
