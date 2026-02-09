use crate::ui::themes::ColorPalette;
use ratatui::Frame;
pub mod help_screen;
use crate::ui::screens::help_screen::HelpScreen;
pub mod quit_screen;
use crate::ui::screens::quit_screen::QuitScreen;
pub mod terminal_screen;
use crate::ui::screens::terminal_screen::TerminalScreen;
pub mod results_screen;
use crate::ui::screens::results_screen::ResultsScreen;

#[derive(Debug)]
pub enum Screen {
    Terminal(TerminalScreen),
    Results(ResultsScreen),
    Help(HelpScreen),
    Exiting(QuitScreen),
}

impl Default for Screen {
    fn default() -> Self {
        Screen::Terminal(TerminalScreen::default())
    }
}

pub trait ScreenRenderable {
    fn render(&self, frame: &mut Frame, theme: &ColorPalette);
}
