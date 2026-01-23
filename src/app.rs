use crate::table::TableView;
use crate::terminal::SqlTerminal;
use crate::ui::ui;
use crate::{sql_session::SqlSession, table};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
};
use ratatui::{DefaultTerminal, Frame};
use std::io;
// Handle screen states
#[derive(Debug, Default)]
pub enum Screen {
    #[default]
    Main,
    Results,
    Help,
    Exiting,
}

pub struct App {
    pub sql_path: String,
    session: SqlSession,
    pub current_screen: Screen,
    pub sql_terminal: SqlTerminal,
    pub table_view: Option<TableView>,
    exit: bool,
}

impl App {
    pub fn new(sql_path: String, read_only: bool) -> Self {
        let sql_session = SqlSession::new(sql_path.clone(), read_only);
        App {
            sql_path,
            session: sql_session,
            current_screen: Screen::Main,
            sql_terminal: SqlTerminal::new(),
            table_view: None,
            exit: false,
        }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;

            match self.current_screen {
                Screen::Main => {}
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
                    (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                        self.current_screen = Screen::Exiting
                    }
                    (KeyCode::Char('h'), KeyModifiers::CONTROL) => {
                        self.current_screen = Screen::Help
                    }
                    _ => {}
                }

                if let Screen::Help = self.current_screen {
                    match key_event.code {
                        KeyCode::Esc => self.current_screen = Screen::Main,
                        KeyCode::Char('q') => self.current_screen = Screen::Main,
                        _ => {}
                    }
                }

                if let Screen::Exiting = self.current_screen {
                    match key_event.code {
                        KeyCode::Char('y') => self.exit(),
                        KeyCode::Char('n') => self.current_screen = Screen::Main,
                        _ => {}
                    }
                }

                if let Screen::Results = self.current_screen {
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
                        KeyCode::Char('q') | KeyCode::Esc => self.current_screen = Screen::Main,
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
}
