use crate::{
    actions::Actionable, app::App, ui::screens::Screen,
    ui::screens::terminal_screen::TerminalScreen,
};
use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug)]
pub enum QuitActions {
    ExitApplication,
}

impl Actionable for QuitActions {
    fn take_action(app: &mut App, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('y') => app.exit(),
            KeyCode::Char('n') => app.screen = Screen::Terminal(TerminalScreen::new()),
            _ => {}
        }
    }
}
