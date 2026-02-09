use crate::ui::themes::ColorPalette;
use ratatui::Frame;
pub mod help_screen;
use crate::screens::help_screen::HelpScreen;
use crate::screens::quit_screen::QuitScreen;
pub mod terminal_screen;
use crate::screens::terminal_screen::TerminalScreen;

#[derive(Debug, Default)]
pub enum Screen {
    #[default]
    Terminal(TerminalScreen),
    Results,
    Help(HelpScreen),
    Exiting(QuitScreen),
}

trait ScreenRenderable {
    fn render(&self, frame: &mut Frame, theme: &ColorPalette);
}
