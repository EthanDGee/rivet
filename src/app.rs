use crate::model::notifications::NotificationList;
use crate::sql_session::SqlSession;
use crate::ui::screens::{
    Screen, help_screen::HelpScreen, quit_screen::QuitScreen, results_screen::ResultsScreen,
    terminal_screen::TerminalScreen,
};
use crate::ui::{table::TableView, themes::ColorPalette, ui};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{DefaultTerminal, Frame};
use std::io;

pub const TOOL_NAME: &str = "rivet";

pub struct App {
    pub sql_path: String,
    session: SqlSession,
    pub screen: Screen,
    pub notifications: NotificationList,
    pub theme: ColorPalette,
    exit: bool,
}

impl App {
    pub fn new(sql_path: String, read_only: bool) -> Self {
        let sql_session = SqlSession::new(sql_path.clone(), read_only);
        App {
            sql_path,
            session: sql_session,
            screen: Screen::Terminal(TerminalScreen::new()),
            notifications: NotificationList::new(),
            theme: ColorPalette::nord(),
            exit: false,
        }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;

            self.notifications.remove_expired();

            match &mut self.screen {
                Screen::Terminal(_terminal_screen) => {}
                Screen::Results(_results_screen) => {}
                Screen::Help(_help_screen) => {}
                Screen::Exiting(_quit_screen) => {}
            }
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_global_keys(key_event);

                match self.screen {
                    Screen::Terminal(_) => self.handle_terminal_keys(key_event),
                    Screen::Results(_) => self.handle_results_keys(key_event),
                    Screen::Help(_) => self.handle_help_keys(key_event),
                    Screen::Exiting(_) => self.handle_exiting_keys(key_event),
                }
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_global_keys(&mut self, key_event: KeyEvent) {
        match (key_event.code, key_event.modifiers) {
            (KeyCode::Char('s'), KeyModifiers::CONTROL) => {
                self.session.commit();
                self.notifications
                    .notify("Save", "Changes to database saved successfully.")
            }
            (KeyCode::Char('r'), KeyModifiers::CONTROL) => {
                self.session.rollback();
                self.notifications
                    .notify("Rollback", "Staged changes successfully reverted.")
            }
            (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                self.screen = Screen::Exiting(QuitScreen::new())
            }
            (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                self.screen = Screen::Exiting(QuitScreen::new())
            }
            (KeyCode::Char('h'), KeyModifiers::CONTROL) => {
                self.screen = Screen::Help(HelpScreen::new())
            }
            _ => {}
        }
    }

    fn handle_terminal_keys(&mut self, key_event: KeyEvent) {
        if let Screen::Terminal(terminal_screen) = &mut self.screen {
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
                    if let Some(new_screen) = self.execute_command() {
                        self.screen = new_screen;
                    }
                }
                _ => {}
            }
        }
    }

    fn handle_results_keys(&mut self, key_event: KeyEvent) {
        if let Screen::Results(results_screen) = &mut self.screen {
            //handle table navigation
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
                    self.screen = Screen::Terminal(TerminalScreen::new());
                }
                _ => {}
            }
        }
    }

    fn handle_help_keys(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.screen = Screen::Terminal(TerminalScreen::new());
            }
            _ => {}
        }
    }

    fn handle_exiting_keys(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('y') => self.exit(),
            KeyCode::Char('n') => self.screen = Screen::Terminal(TerminalScreen::new()),
            _ => {}
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        ui(frame, self)
    }

    // App Specific Functionality.
    fn exit(&mut self) {
        // TODO: flush cache to prevent unwanted changes being saved in future sessions

        self.exit = true;
    }

    fn execute_command(&mut self) -> Option<Screen> {
        // This command can only be executed from the Terminal screen
        let Screen::Terminal(terminal_screen) = &mut self.screen else {
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
            let column_names: Vec<String> = self
                .session
                .extract_column_names(&query)
                .unwrap_or_default();
            match self.session.select(&query) {
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
            match self.session.execute(&query) {
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
}
