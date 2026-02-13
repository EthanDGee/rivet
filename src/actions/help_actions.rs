use crate::{
    actions::Actionable, app::App, ui::screens::Screen,
    ui::screens::terminal_screen::TerminalScreen,
};
use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug)]
pub enum HelpActions {
    ExitHelp,
}

impl Actionable for HelpActions {
    fn take_action(app: &mut App, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                app.screen = Screen::Terminal(TerminalScreen::new());
            }
            _ => {}
        }
    }
}
