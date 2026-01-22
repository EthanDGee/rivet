use crate::sql_session::SqlSession;
use crate::ui::ui;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{DefaultTerminal, Frame};
use std::io;
// Handle screen states
#[derive(Debug, Default)]
pub enum Screens {
    #[default]
    Main,
    Results,
    Help,
    Exiting,
}

pub struct App {
    pub sql_path: String,
    session: SqlSession,
    pub current_screen: Screens,
    exit: bool,
}

impl App {
    pub fn new(sql_path: String) -> Self {
        let sql_session = SqlSession::new(sql_path.clone());
        App {
            sql_path,
            session: sql_session,
            current_screen: Screens::Main,
            exit: false,
        }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;

            match self.current_screen {
                Screens::Main => {}
                Screens::Results => {}
                Screens::Help => {}
                Screens::Exiting => {}
            }
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                // Application Wide Commands
                match (key_event.code, key_event.modifiers) {
                    (KeyCode::Char('q'), KeyModifiers::CONTROL) => self.exit(),
                    (KeyCode::Char('s'), KeyModifiers::CONTROL) => self.session.commit(),
                    (KeyCode::Char('h'), KeyModifiers::CONTROL) => {
                        self.current_screen = Screens::Help
                    }
                    _ => {}
                }

                if let Screens::Help = self.current_screen {
                    match key_event.code {
                        KeyCode::Esc => self.current_screen = Screens::Main,
                        KeyCode::Char('q') => self.current_screen = Screens::Main,
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
