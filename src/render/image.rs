use image::DynamicImage;
use ratatui::layout::Rect;
use ratatui::Frame;
use ratatui_image::picker::{Picker, ProtocolType};
use ratatui_image::protocol::StatefulProtocol;
use ratatui_image::{FontSize, Resize, StatefulImage};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ImageProtocol {
    #[default]
    Auto,
    Kitty,
    Sixel,
    Iterm2,
    Halfblocks,
}

impl ImageProtocol {
    pub fn resolved(self) -> ProtocolType {
        match self {
            Self::Auto => auto_protocol(),
            Self::Kitty => ProtocolType::Kitty,
            Self::Sixel => ProtocolType::Sixel,
            Self::Iterm2 => ProtocolType::Iterm2,
            Self::Halfblocks => ProtocolType::Halfblocks,
        }
    }
}

pub fn create_picker(protocol: ImageProtocol) -> Picker {
    let font_size = terminal_font_size();
    #[allow(deprecated)]
    let mut picker = Picker::from_fontsize(font_size);
    picker.set_protocol_type(protocol.resolved());
    picker
}

fn terminal_font_size() -> FontSize {
    crossterm::terminal::window_size()
        .ok()
        .and_then(|size| {
            if size.columns == 0 || size.rows == 0 || size.width == 0 || size.height == 0 {
                return None;
            }

            Some(FontSize::new(
                (size.width / size.columns).max(1),
                (size.height / size.rows).max(1),
            ))
        })
        .unwrap_or(FontSize::new(10, 20))
}

fn auto_protocol() -> ProtocolType {
    let term = std::env::var("TERM")
        .unwrap_or_default()
        .to_ascii_lowercase();
    let program = std::env::var("TERM_PROGRAM")
        .unwrap_or_default()
        .to_ascii_lowercase();

    if std::env::var_os("KITTY_WINDOW_ID").is_some() || term.contains("kitty") {
        return ProtocolType::Kitty;
    }

    if program == "iterm.app"
        || term.contains("iterm")
        || program == "wezterm"
        || std::env::var_os("WEZTERM_PANE").is_some()
    {
        return ProtocolType::Iterm2;
    }

    if term.contains("sixel")
        || term.contains("foot")
        || term.contains("mlterm")
        || program == "mlterm"
        || program == "foot"
    {
        return ProtocolType::Sixel;
    }

    ProtocolType::Halfblocks
}

pub struct ImageState {
    picker: Picker,
    protocol: Option<StatefulProtocol>,
    signature: Option<u64>,
}

impl ImageState {
    pub fn new(picker: Picker) -> Self {
        Self {
            picker,
            protocol: None,
            signature: None,
        }
    }

    pub fn protocol_name(&self) -> &'static str {
        use ratatui_image::picker::ProtocolType;

        match self.picker.protocol_type() {
            ProtocolType::Halfblocks => "halfblocks",
            ProtocolType::Sixel => "sixel",
            ProtocolType::Kitty => "kitty",
            ProtocolType::Iterm2 => "iterm2",
        }
    }

    pub fn square_area(&self, area: Rect) -> Rect {
        let font = self.picker.font_size();
        let font_width = u32::from(font.width.max(1));
        let font_height = u32::from(font.height.max(1));

        let width = u32::from(area.width);
        let height = u32::from(area.height);
        let by_height = height * font_height / font_width;
        let columns = width.min(by_height).max(1);
        let rows = (columns * font_width / font_height).max(1);
        let columns = columns.min(width).max(1) as u16;
        let rows = rows.min(height).max(1) as u16;

        Rect::new(
            area.x + (area.width - columns) / 2,
            area.y + (area.height - rows) / 2,
            columns,
            rows,
        )
    }

    pub fn signature(&self) -> Option<u64> {
        self.signature
    }

    pub fn update(&mut self, image: DynamicImage, signature: u64) {
        if self.signature != Some(signature) {
            self.protocol = Some(self.picker.new_resize_protocol(image));
            self.signature = Some(signature);
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        if let Some(protocol) = self.protocol.as_mut() {
            frame.render_stateful_widget(
                StatefulImage::new().resize(Resize::Fit(None)),
                area,
                protocol,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui_image::picker::Picker;

    fn state() -> ImageState {
        let mut picker = Picker::halfblocks();
        picker.set_protocol_type(ProtocolType::Halfblocks);
        ImageState::new(picker)
    }

    #[test]
    fn explicit_protocols_resolve_without_terminal_queries() {
        assert_eq!(ImageProtocol::Kitty.resolved(), ProtocolType::Kitty);
        assert_eq!(ImageProtocol::Sixel.resolved(), ProtocolType::Sixel);
        assert_eq!(ImageProtocol::Iterm2.resolved(), ProtocolType::Iterm2);
        assert_eq!(
            ImageProtocol::Halfblocks.resolved(),
            ProtocolType::Halfblocks
        );
    }

    #[test]
    fn square_area_fits_inside_the_available_rect() {
        let image = state();
        let area = Rect::new(5, 7, 100, 40);
        let square = image.square_area(area);

        assert!(square.width <= area.width);
        assert!(square.height <= area.height);
        assert!(square.x >= area.x);
        assert!(square.y >= area.y);
    }

    #[test]
    fn square_area_is_centered() {
        let image = state();
        let area = Rect::new(0, 0, 80, 24);
        let square = image.square_area(area);

        assert_eq!(square.x, (area.width - square.width) / 2);
        assert_eq!(square.y, (area.height - square.height) / 2);
    }
}
