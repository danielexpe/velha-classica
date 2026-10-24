use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorMode {
    #[default]
    Ansi16,
    Ansi256,
    TrueColor,
    Monochrome,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorChoice {
    #[default]
    Auto,
    TrueColor,
    Ansi256,
    Ansi16,
    Never,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ColorEnv<'a> {
    pub no_color: bool,
    pub colorterm: Option<&'a str>,
    pub term: Option<&'a str>,
}

pub fn detect_color(choice: ColorChoice, env: ColorEnv<'_>) -> ColorMode {
    if env.no_color {
        return ColorMode::Monochrome;
    }

    match choice {
        ColorChoice::Auto => auto_color(env),
        ColorChoice::TrueColor => ColorMode::TrueColor,
        ColorChoice::Ansi256 => ColorMode::Ansi256,
        ColorChoice::Ansi16 => ColorMode::Ansi16,
        ColorChoice::Never => ColorMode::Monochrome,
    }
}

fn auto_color(env: ColorEnv<'_>) -> ColorMode {
    if let Some(colorterm) = env.colorterm {
        if colorterm.eq_ignore_ascii_case("truecolor") || colorterm.eq_ignore_ascii_case("24bit") {
            return ColorMode::TrueColor;
        }
    }

    if let Some(term) = env.term {
        let term = term.to_ascii_lowercase();

        if term.contains("direct") || term.contains("truecolor") {
            return ColorMode::TrueColor;
        }

        if term.contains("256color") {
            return ColorMode::Ansi256;
        }
    }

    ColorMode::Ansi16
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Rgb {
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    pub const fn to_color(self) -> Color {
        Color::Rgb(self.red, self.green, self.blue)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    SurfaceWoodDeep,
    SurfaceWood,
    SurfaceWoodMid,
    SurfaceWoodLight,
    SurfacePaper,
    SurfacePaperLight,
    SurfacePaperShadow,
    TextOnPaper,
    TextOnPaperMuted,
    TextOnWood,
    TextOnWoodMuted,
    MarkX,
    MarkO,
    Brass,
    BrassBright,
    Success,
    Danger,
    Focus,
    Shadow,
    Overlay,
}

impl Token {
    pub const fn rgb(self) -> Rgb {
        match self {
            Self::SurfaceWoodDeep => Rgb::new(0x1E, 0x14, 0x0E),
            Self::SurfaceWood => Rgb::new(0x2B, 0x1C, 0x12),
            Self::SurfaceWoodMid => Rgb::new(0x4A, 0x2F, 0x1D),
            Self::SurfaceWoodLight => Rgb::new(0x6B, 0x47, 0x28),
            Self::SurfacePaper => Rgb::new(0xEA, 0xD9, 0xB7),
            Self::SurfacePaperLight => Rgb::new(0xF7, 0xE9, 0xCD),
            Self::SurfacePaperShadow => Rgb::new(0xCD, 0xB8, 0x8F),
            Self::TextOnPaper => Rgb::new(0x2A, 0x21, 0x18),
            Self::TextOnPaperMuted => Rgb::new(0x6E, 0x5A, 0x43),
            Self::TextOnWood => Rgb::new(0xF2, 0xE5, 0xCC),
            Self::TextOnWoodMuted => Rgb::new(0xB9, 0xA7, 0x8E),
            Self::MarkX => Rgb::new(0x28, 0x4B, 0x5A),
            Self::MarkO => Rgb::new(0x8E, 0x3B, 0x2F),
            Self::Brass => Rgb::new(0xC7, 0xA2, 0x3A),
            Self::BrassBright => Rgb::new(0xE6, 0xC7, 0x66),
            Self::Success => Rgb::new(0x6F, 0x8A, 0x56),
            Self::Danger => Rgb::new(0xA9, 0x44, 0x38),
            Self::Focus => Rgb::new(0xE6, 0xC7, 0x66),
            Self::Shadow => Rgb::new(0x12, 0x0B, 0x06),
            Self::Overlay => Rgb::new(0x0B, 0x07, 0x04),
        }
    }

    pub const fn ansi256(self) -> u8 {
        match self {
            Self::SurfaceWoodDeep => 234,
            Self::SurfaceWood => 235,
            Self::SurfaceWoodMid => 94,
            Self::SurfaceWoodLight => 137,
            Self::SurfacePaper => 230,
            Self::SurfacePaperLight => 231,
            Self::SurfacePaperShadow => 180,
            Self::TextOnPaper => 235,
            Self::TextOnPaperMuted => 240,
            Self::TextOnWood => 230,
            Self::TextOnWoodMuted => 180,
            Self::MarkX => 24,
            Self::MarkO => 124,
            Self::Brass => 178,
            Self::BrassBright => 220,
            Self::Success => 64,
            Self::Danger => 160,
            Self::Focus => 220,
            Self::Shadow => 232,
            Self::Overlay => 233,
        }
    }

    pub const fn ansi16(self) -> Color {
        match self {
            Self::SurfaceWoodDeep | Self::SurfaceWood | Self::Shadow | Self::Overlay => {
                Color::Black
            }
            Self::SurfaceWoodMid | Self::TextOnPaperMuted => Color::DarkGray,
            Self::SurfaceWoodLight | Self::SurfacePaperShadow | Self::TextOnWoodMuted => {
                Color::Gray
            }
            Self::SurfacePaper | Self::SurfacePaperLight | Self::TextOnWood => Color::White,
            Self::TextOnPaper => Color::Black,
            Self::MarkX => Color::LightCyan,
            Self::MarkO | Self::Danger => Color::LightRed,
            Self::Brass => Color::Yellow,
            Self::BrassBright | Self::Focus => Color::LightYellow,
            Self::Success => Color::Green,
        }
    }
}

pub fn color(token: Token, mode: ColorMode) -> Color {
    match mode {
        ColorMode::Monochrome => Color::Reset,
        ColorMode::Ansi16 => token.ansi16(),
        ColorMode::Ansi256 => Color::Indexed(token.ansi256()),
        ColorMode::TrueColor => token.rgb().to_color(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_color_has_priority() {
        let env = ColorEnv {
            no_color: true,
            colorterm: Some("truecolor"),
            term: Some("xterm-256color"),
        };

        assert_eq!(
            detect_color(ColorChoice::TrueColor, env),
            ColorMode::Monochrome
        );
    }

    #[test]
    fn never_disables_color() {
        let env = ColorEnv {
            no_color: false,
            colorterm: Some("truecolor"),
            term: Some("xterm-256color"),
        };

        assert_eq!(detect_color(ColorChoice::Never, env), ColorMode::Monochrome);
    }

    #[test]
    fn auto_detects_truecolor_from_colorterm() {
        let env = ColorEnv {
            no_color: false,
            colorterm: Some("24bit"),
            term: Some("xterm-256color"),
        };

        assert_eq!(detect_color(ColorChoice::Auto, env), ColorMode::TrueColor);
    }

    #[test]
    fn auto_detects_truecolor_from_term() {
        let env = ColorEnv {
            no_color: false,
            colorterm: None,
            term: Some("xterm-direct"),
        };

        assert_eq!(detect_color(ColorChoice::Auto, env), ColorMode::TrueColor);
    }

    #[test]
    fn auto_detects_256_colors() {
        let env = ColorEnv {
            no_color: false,
            colorterm: None,
            term: Some("xterm-256color"),
        };

        assert_eq!(detect_color(ColorChoice::Auto, env), ColorMode::Ansi256);
    }

    #[test]
    fn auto_falls_back_to_16_colors() {
        let env = ColorEnv {
            no_color: false,
            colorterm: None,
            term: Some("xterm"),
        };

        assert_eq!(detect_color(ColorChoice::Auto, env), ColorMode::Ansi16);
    }

    #[test]
    fn explicit_choices_are_respected() {
        let env = ColorEnv::default();

        assert_eq!(detect_color(ColorChoice::Ansi256, env), ColorMode::Ansi256);
        assert_eq!(detect_color(ColorChoice::Ansi16, env), ColorMode::Ansi16);
        assert_eq!(
            detect_color(ColorChoice::TrueColor, env),
            ColorMode::TrueColor
        );
    }

    #[test]
    fn monochrome_uses_reset_color() {
        assert_eq!(color(Token::Brass, ColorMode::Monochrome), Color::Reset);
    }

    #[test]
    fn truecolor_uses_exact_palette() {
        assert_eq!(
            color(Token::MarkX, ColorMode::TrueColor),
            Color::Rgb(0x28, 0x4B, 0x5A)
        );
        assert_eq!(
            color(Token::SurfaceWoodDeep, ColorMode::TrueColor),
            Color::Rgb(0x1E, 0x14, 0x0E)
        );
    }

    #[test]
    fn indexed_modes_use_expected_values() {
        assert_eq!(
            color(Token::SurfacePaper, ColorMode::Ansi256),
            Color::Indexed(230)
        );
        assert_eq!(
            color(Token::BrassBright, ColorMode::Ansi16),
            Color::LightYellow
        );
    }
}
