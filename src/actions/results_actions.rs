use crate::{
    actions::Actionable,
    app::App,
    ui::screens::{Screen, terminal_screen::TerminalScreen},
};
use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug)]
pub enum ResultActions {
    MoveCursorRight,
    MoveCursorLeft,
    MoveCursorUp,
    MoveCursorDown,
    ExitResults,
}

impl Actionable for ResultActions {
    fn take_action(app: &mut App, key_event: KeyEvent) {
        if let Screen::Results(results_screen) = &mut app.screen {
            //handle table navigation if the tableview is loaded
            if let Some(table_view) = &mut results_screen.table_view {
                match key_event.code {
                    KeyCode::Char('j') | KeyCode::Down => table_view.next_row(),
                    KeyCode::Char('k') | KeyCode::Up => table_view.previous_row(),
                    KeyCode::Char('h') | KeyCode::Left => table_view.previous_column(),
                    KeyCode::Char('l') | KeyCode::Right => table_view.next_column(),
                    _ => {}
                }
            }
            // non navigation related functionality
            match key_event.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    app.screen = Screen::Terminal(TerminalScreen::new());
                }
                _ => {}
            }
        }
    }
}
