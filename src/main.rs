use std::io;
mod app;
mod constants;
use app::App;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

fn main() -> io::Result<()> {
    let mut app: App = App::new("test.sqlite3".to_string());
    ratatui::run(|terminal| app.run(terminal))
}
