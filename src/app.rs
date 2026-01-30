use crate::sql_session::SqlSession;
use crate::ui::{
    notifications::NotificationList, screen::Screen, table::TableView, terminal::SqlTerminal,
    themes::ColorPalette, ui,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{DefaultTerminal, Frame};
use std::io;

pub const TOOL_NAME: &str = "rust-cli-tool";

pub struct App {
    pub sql_path: String,
    session: SqlSession,
    pub screen: Screen,
    pub sql_terminal: SqlTerminal,
    pub table_view: Option<TableView>,
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
            screen: Screen::Terminal,
            sql_terminal: SqlTerminal::new(),
            table_view: None,
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

            match self.screen {
                Screen::Terminal => {}
                Screen::Results => {}
                Screen::Help => {}
                Screen::Exiting => {}
            }
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_global_keys(key_event);

                match self.screen {
                    Screen::Terminal => self.handle_terminal_keys(key_event),
                    Screen::Results => self.handle_results_keys(key_event),
                    Screen::Help => self.handle_help_keys(key_event),
                    Screen::Exiting => self.handle_exiting_keys(key_event),
                }
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_global_keys(&mut self, key_event: KeyEvent) {
        match (key_event.code, key_event.modifiers) {
            (KeyCode::Char('s'), KeyModifiers::CONTROL) => {
                self.notifications
                    .notify("Save", "Changes to database saved successfully.");
                self.session.commit()
            }
            (KeyCode::Char('q'), KeyModifiers::CONTROL) => self.screen = Screen::Exiting,
            (KeyCode::Char('c'), KeyModifiers::CONTROL) => self.screen = Screen::Exiting,
            (KeyCode::Char('h'), KeyModifiers::CONTROL) => self.screen = Screen::Help,
            _ => {}
        }
    }

    fn handle_terminal_keys(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Up => self.sql_terminal.decrement_history(),
            KeyCode::Down => self.sql_terminal.increment_history(),
            KeyCode::Left => self.sql_terminal.move_cursor_left(),
            KeyCode::Right => self.sql_terminal.move_cursor_right(),
            KeyCode::Char(to_insert) => self.sql_terminal.enter_char(to_insert),
            KeyCode::Backspace => self.sql_terminal.delete_char(),
            KeyCode::Delete => {
                //TODO: resolve issues with delete turning into backspace at end of
                //line
                self.sql_terminal.move_cursor_right();
                self.sql_terminal.delete_char();
            }
            KeyCode::Enter => {
                self.execute_command();
            }
            _ => {}
        }
    }

    fn handle_results_keys(&mut self, key_event: KeyEvent) {
        //handle table navigation
        if let Some(table_view) = &mut self.table_view {
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
                self.screen = Screen::Terminal;
                self.table_view = None;
            }
            _ => {}
        }
    }

    fn handle_help_keys(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.screen = Screen::Terminal;
                self.table_view = None;
            }
            _ => {}
        }
    }

    fn handle_exiting_keys(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('y') => self.exit(),
            KeyCode::Char('n') => self.screen = Screen::Terminal,
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

    fn execute_command(&mut self) {
        let query = self.sql_terminal.input.to_string();
        if query.is_empty() {
            return;
        }

        self.sql_terminal.add_log_line(format!("> {}", query));

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
                        self.sql_terminal
                            .add_log_line("Query returned 0 rows".to_string());
                    } else {
                        self.table_view = Some(TableView::new(column_names, data));
                        self.screen = Screen::Results;
                    }
                }
                Err(e) => {
                    self.sql_terminal.add_log_line(format!("Error: {}", e));
                }
            }
        } else {
            match self.session.execute(&query) {
                Ok(changes) => {
                    self.sql_terminal
                        .add_log_line(format!("{} changes.", changes));
                }
                Err(e) => {
                    self.sql_terminal.add_log_line(format!("Error: {}", e));
                }
            }
        }

        //update history for navigation
        self.sql_terminal.add_command();
    }
}
