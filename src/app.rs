use crate::sql_session::SqlSession;
use crate::ui::ui;
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
    exit: bool,
}

impl App {
    pub fn new(sql_path: String) -> Self {
        let sql_session = SqlSession::new(sql_path.clone());
        App {
            sql_path,
            session: sql_session,
            current_screen: Screen::Main,
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
