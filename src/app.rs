use crate::constants::TOOL_NAME;
use crate::sql_session::SqlSession;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};
use std::{io, process::exit};

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
    sql_path: String,
    session: SqlSession,
    current_screen: Screens,
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

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match (key_event.code, key_event.modifiers) {
                    (KeyCode::Char('q'), KeyModifiers::CONTROL) => self.exit(),
                    (KeyCode::Char('s'), KeyModifiers::CONTROL) => self.session.commit(),
                    _ => {}
                }
            }
            _ => {}
        };
        Ok(())
    }

    // App Specific Functionality.
    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(TOOL_NAME.bold());
        let instructions = Line::from(vec![" Quit ".into(), "<C-Q> ".blue().bold()]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let db_info = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.sql_path.to_string().yellow(),
        ])]);

        Paragraph::new(db_info)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
