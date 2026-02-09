use ratatui::{
    Frame,
    style::Stylize,
    widgets::{Block, Padding, Paragraph},
};

use crate::app::TOOL_NAME;
use super::ScreenRenderable;
use crate::ui::themes::ColorPalette;
use crate::ui::utils::floating_window;

#[derive(Debug, Default)]
pub struct QuitScreen;

impl QuitScreen {
    pub fn new() -> Self {
        QuitScreen
    }
}

impl ScreenRenderable for QuitScreen {
    fn render(&self, frame: &mut Frame, theme: &ColorPalette) {
        let floating_window_rect = floating_window(frame, theme);

        let confirmation = Paragraph::new(format!("Quit {} Session? y/n", TOOL_NAME))
            .centered()
            .block(Block::default().padding(Padding::uniform(2)))
            .bold()
            .fg(theme.body_text);

        frame.render_widget(confirmation, floating_window_rect);
    }
}
