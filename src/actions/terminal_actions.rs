use crate::{
    actions::Actionable,
    app::App,
    ui::{
        screens::{Screen, results_screen::ResultsScreen},
        table::TableView,
    },
};
use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug)]
pub enum TerminalActions {
    MoveHistoryForward,
    MoveHistoryBackward,
    MoveCursorRight,
    MoveCursorLeft,
    InputCharacter,
    Backspace,
    Delete,
    EnterCommand,
}

fn execute_command(app: &mut App) -> Option<Screen> {
    // This command can only be executed from the Terminal screen
    let Screen::Terminal(terminal_screen) = &mut app.screen else {
        return None;
    };

    let query = terminal_screen.input.to_string();
    if query.is_empty() {
        terminal_screen.add_command();
        return None;
    }

    terminal_screen.add_log_line(format!("> {}", query));

    // Toggle operation for select vs other operations
    if query
        .split_whitespace()
        .next()
        .is_some_and(|x| x.to_ascii_uppercase().eq("SELECT"))
    {
        let column_names: Vec<String> =
            app.session.extract_column_names(&query).unwrap_or_default();
        match app.session.select(&query) {
            Ok(data) => {
                if data.is_empty() {
                    terminal_screen.add_log_line("Query returned 0 rows".to_string());
                    terminal_screen.add_command();
                    None
                } else {
                    let mut results_screen = ResultsScreen::new();
                    results_screen.table_view = Some(TableView::new(column_names, data));
                    terminal_screen.add_command();
                    Some(Screen::Results(results_screen))
                }
            }
            Err(e) => {
                terminal_screen.add_log_line(format!("Error: {}", e));
                terminal_screen.add_command();
                None
            }
        }
    } else {
        match app.session.execute(&query) {
            Ok(changes) => {
                terminal_screen.add_log_line(format!("{} changes.", changes));
            }
            Err(e) => {
                terminal_screen.add_log_line(format!("Error: {}", e));
            }
        }
        terminal_screen.add_command();
        None
    }
}

impl Actionable for TerminalActions {
    fn take_action(app: &mut App, key_event: KeyEvent) {
        if let Screen::Terminal(terminal_screen) = &mut app.screen {
            match key_event.code {
                KeyCode::Up => terminal_screen.decrement_history(),
                KeyCode::Down => terminal_screen.increment_history(),
                KeyCode::Left => terminal_screen.move_cursor_left(),
                KeyCode::Right => terminal_screen.move_cursor_right(),
                KeyCode::Char(to_insert) => terminal_screen.enter_char(to_insert),
                KeyCode::Backspace => terminal_screen.delete_char(),
                KeyCode::Delete => {
                    //TODO: resolve issues with delete turning into backspace at end of
                    //line
                    terminal_screen.move_cursor_right();
                    terminal_screen.delete_char();
                }
                KeyCode::Enter => {
                    if let Some(new_screen) = execute_command(app) {
                        app.screen = new_screen;
                    }
                }
                _ => {}
            }
        }
    }
}
