use ratatui::{
    Frame,
    style::Stylize,
    widgets::{Block, Padding, Paragraph},
};

use crate::screens::ScreenRenderable;
use crate::ui::themes::ColorPalette;
use crate::ui::utils::floating_window;

#[derive(Debug, Default)]
pub struct HelpScreen;

impl HelpScreen {
    pub fn new() -> Self {
        HelpScreen
    }
}

impl ScreenRenderable for HelpScreen {
    fn render(&self, frame: &mut Frame, theme: &ColorPalette) {
        let floating_window_rect = floating_window(frame, theme);
        let commands = Paragraph::new("HELP")
            .centered()
            .block(Block::default().padding(Padding::uniform(2)))
            .fg(theme.body_text);

        frame.render_widget(commands, floating_window_rect);
    }
}
