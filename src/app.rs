use crate::actions::Actions;
use crate::model::notifications::NotificationList;
use crate::model::sql_session::SqlSession;
use crate::ui::screens::{Screen, terminal_screen::TerminalScreen};
use crate::ui::{themes::ColorPalette, ui};
use crossterm::event::{self, Event, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use std::io;

pub const TOOL_NAME: &str = "rivet";

pub struct App {
    pub sql_path: String,
    pub session: SqlSession,
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

            // Handle user actions
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    <Actions>::handle_actions(self, key_event);
                }
                _ => {}
            };

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

    fn draw(&mut self, frame: &mut Frame) {
        ui(frame, self)
    }

    // App Specific Functionality.
    pub fn exit(&mut self) {
        self.exit = true;
    }
}
