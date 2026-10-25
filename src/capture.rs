use std::path::Path;

use tiny_skia::Pixmap;

use crate::app::App;
use crate::i18n::Language;
use crate::render::fb;

pub const WIDTH: u32 = fb::WIDTH;
pub const HEIGHT: u32 = fb::HEIGHT;

pub fn render(language: Language) -> Pixmap {
    let app = App::preview(language);
    let mut renderer = fb::Renderer::new(WIDTH, HEIGHT);

    renderer.capture(&app)
}

pub fn write(path: &Path, language: Language) -> Result<(), String> {
    let pixmap = render(language);
    let png = pixmap.encode_png().map_err(|error| error.to_string())?;

    std::fs::write(path, png).map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_has_the_expected_canvas_size() {
        let frame = render(Language::PtBr);

        assert_eq!((frame.width(), frame.height()), (WIDTH, HEIGHT));
    }

    #[test]
    fn languages_produce_different_stamps() {
        assert_ne!(render(Language::PtBr).data(), render(Language::En).data());
    }

    #[test]
    fn frame_can_be_written() {
        let path = std::env::temp_dir().join(format!("velha-capture-{}.png", std::process::id()));

        write(&path, Language::PtBr).unwrap();
        let bytes = std::fs::read(&path).unwrap();
        let _ = std::fs::remove_file(&path);

        assert!(bytes.starts_with(b"\x89PNG"));
    }

    #[test]
    fn preview_is_deterministic() {
        assert_eq!(render(Language::En).data(), render(Language::En).data());
    }
}
