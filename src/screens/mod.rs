use crate::ui::themes::ColorPalette;
use ratatui::Frame;

#[derive(Debug, Default)]
pub enum Screen {
    #[default]
    Terminal,
    Results,
    Help,
    Exiting,
}

trait ScreenRenderable {
    fn render(&self, frame: &mut Frame, theme: &ColorPalette);
}
