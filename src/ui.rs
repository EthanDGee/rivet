use crate::app::{App, Screen};
use crate::constants::TOOL_NAME;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, Padding, Paragraph},
};
use std::format;

pub struct ColorPalette {
    pub title: Color,
    pub outer_border: Color,
    pub inner_border: Color,
    pub header_text: Color,
    pub body_text: Color,
    pub highlight: Color,
}

impl ColorPalette {
    pub fn tokyo_night() -> Self {
        Self {
            title: Color::from_u32(0xff9e64),        // orange
            outer_border: Color::from_u32(0xbb9af7), // purple
            inner_border: Color::from_u32(0x7aa2f7), // blue
            header_text: Color::from_u32(0x9ece6a),  // green
            body_text: Color::from_u32(0xc0caf5),    // foreground
            highlight: Color::from_u32(0x73daca),    // cyan
        }
    }
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // return the middle chunk
}

pub fn floating_window(frame: &mut Frame, theme: &ColorPalette) -> Rect {
    let window = centered_rect(75, 75, frame.area());
    let border = Block::bordered()
        .border_style(Style::default().fg(theme.inner_border))
        .border_set(border::THICK);

    frame.render_widget(border, window);

    window
}

pub fn ui(frame: &mut Frame, app: &App) {
    let title = Line::from(
        format!("{}({})", TOOL_NAME, app.sql_path)
            .bold()
            .fg(app.theme.title),
    );
    let instructions = Line::from(vec![
        " Help ".into(),
        "<C-H>".fg(app.theme.highlight).bold(),
        " Save ".into(),
        "<C-S>".fg(app.theme.highlight).bold(),
        " Quit ".into(),
        "<C-Q> ".fg(app.theme.highlight).bold(),
    ]);

    let block = Block::bordered()
        .title(title.centered())
        .style(Style::default().fg(app.theme.body_text))
        .border_style(Style::default().fg(app.theme.outer_border))
        .title_bottom(instructions.centered())
        .border_set(border::ROUNDED);

    frame.render_widget(block, frame.area());

    if let Screen::Terminal = app.screen {
        let terminal_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Max(90), Constraint::Min(5)])
            .split(frame.area());

        let command_line = Paragraph::new(format!("> {}", app.sql_terminal.input.clone()))
            .fg(app.theme.header_text)
            .block(Block::default().padding(Padding::uniform(2)));

        frame.render_widget(command_line, terminal_layout[1]);
        return;
    }

    if let Screen::Help = app.screen {
        let floating_window_rect = floating_window(frame, &app.theme);
        let commands = Paragraph::new("HELP")
            .centered()
            .block(Block::default().padding(Padding::uniform(2)))
            .fg(app.theme.body_text);

        frame.render_widget(commands, floating_window_rect);
        return;
    }

    if let Screen::Exiting = app.screen {
        let floating_window_rect = floating_window(frame, &app.theme);

        let confirmation = Paragraph::new(format!("Quit {} Session? y/n", TOOL_NAME))
            .centered()
            .block(Block::default().padding(Padding::uniform(2)))
            .bold()
            .fg(app.theme.body_text);

        frame.render_widget(confirmation, floating_window_rect);
    }
}

