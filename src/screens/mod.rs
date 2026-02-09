use crate::ui::themes::ColorPalette;
use ratatui::Frame;
pub mod help_screen;
use crate::screens::help_screen::HelpScreen;
use crate::screens::quit_screen::QuitScreen;

#[derive(Debug, Default)]
pub enum Screen {
    #[default]
    Terminal,
    Results,
    Help(HelpScreen),
    Exiting(QuitScreen),
}

trait ScreenRenderable {
    fn render(&self, frame: &mut Frame, theme: &ColorPalette);
}

