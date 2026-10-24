use clap::{Parser, ValueEnum};

use crate::i18n::Language;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Parser)]
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

    #[arg(long, value_name = "N", help = "Seed for reproducible AI moves")]
    pub seed: Option<u64>,
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
