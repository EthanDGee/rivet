use crate::sql_session::SqlSession;
use crate::table::TableView;
use crate::terminal::SqlTerminal;
use crate::ui::{ui, ColorPalette};
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{DefaultTerminal, Frame};
use std::io;

// Handle screen states
#[derive(Debug, Default)]
pub enum Screen {
    #[default]
    Terminal,
    Results,
    Help,
    Exiting,
}

pub struct App {
    pub sql_path: String,
    session: SqlSession,
    pub screen: Screen,
    pub sql_terminal: SqlTerminal,
    pub table_view: Option<TableView>,
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
            theme: ColorPalette::tokyo_night(),
            exit: false,
        }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;

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
                // Application Wide Commands
                match (key_event.code, key_event.modifiers) {
                    (KeyCode::Char('s'), KeyModifiers::CONTROL) => self.session.commit(),
                    (KeyCode::Char('q'), KeyModifiers::CONTROL) => self.screen = Screen::Exiting,
                    (KeyCode::Char('h'), KeyModifiers::CONTROL) => self.screen = Screen::Help,
                    _ => {}
                }

                if let Screen::Terminal = self.screen {
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

                if let Screen::Results = self.screen {
                    //handle table navigation
                    if let Some(table_view) = &mut self.table_view {
                        match key_event.code {
                            KeyCode::Char('j') | KeyCode::Up => table_view.next_row(),
                            KeyCode::Char('k') | KeyCode::Down => table_view.previous_row(),
                            KeyCode::Char('h') | KeyCode::Left => table_view.previous_column(),
                            KeyCode::Char('l') | KeyCode::Right => table_view.next_column(),
                            _ => {}
                        }
                    }
                    // non navigation related functionality
                    match key_event.code {
                        KeyCode::Char('q') | KeyCode::Esc => self.screen = Screen::Terminal,
                        _ => {}
                    }
                }

                if let Screen::Help = self.screen {
                    match key_event.code {
                        KeyCode::Esc => self.screen = Screen::Terminal,
                        KeyCode::Char('q') => self.screen = Screen::Terminal,
                        _ => {}
                    }
                }

                if let Screen::Exiting = self.screen {
                    match key_event.code {
                        KeyCode::Char('y') => self.exit(),
                        KeyCode::Char('n') => self.screen = Screen::Terminal,
                        _ => {}
                    }
                }
            }
            _ => {}
        };
        Ok(())
    }
    fn draw(&self, frame: &mut Frame) {
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

        // Toggle operation for select vs other operations
        if query
            .split_whitespace()
            .next()
            .is_some_and(|x| x.to_ascii_uppercase().eq("SELECT"))
        {
            self.session.select(query.clone());
        }
        self.session.execute(query);

        //update history
        self.sql_terminal.add_command();
    }
}
