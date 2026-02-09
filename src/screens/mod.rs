use crate::ui::themes::ColorPalette;
use ratatui::Frame;
pub mod help_screen;
use crate::screens::help_screen::HelpScreen;

#[derive(Debug, Default)]
pub enum Screen {
    #[default]
    Terminal,
    Results,
    Help(HelpScreen),
    Exiting,
}

trait ScreenRenderable {
    fn render(&self, frame: &mut Frame, theme: &ColorPalette);
}

