pub mod notifications;
pub mod screen;
pub mod table;
pub mod terminal;
pub mod themes;

use crate::app::App;
use crate::app::TOOL_NAME;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, Cell, Padding, Paragraph, Row, Scrollbar, ScrollbarOrientation, Table},
};
use screen::Screen;
use std::format;
use std::vec;
use themes::ColorPalette;

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

pub fn ui(frame: &mut Frame, app: &mut App) {
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

    let main_block = Block::bordered()
        .title(title.centered())
        .style(Style::default().fg(app.theme.body_text))
        .border_style(Style::default().fg(app.theme.outer_border))
        .title_bottom(instructions.centered())
        .border_set(border::ROUNDED);

    let main_area = frame.area();
    frame.render_widget(main_block.clone(), main_area);
    let inner_area = main_block.inner(main_area);

    match app.screen {
        Screen::Terminal => render_terminal(frame, app, inner_area),
        Screen::Results => render_results(frame, app, inner_area),
        Screen::Help => render_help(frame, app),
        Screen::Exiting => render_exiting(frame, app),
    }

    // Notifications are rendered last, on top of all other UI elements.
    let notifications = app.notifications.get_notification_widgets(&app.theme);
    if !notifications.is_empty() {
        let area = frame.area();
        const NOTIFICATION_WIDTH: u16 = 32;
        const NOTIFICATION_HEIGHT: u16 = 5;

        for (i, (notification_widget, height)) in notifications
            .iter()
            .zip(
                app.notifications
                    .get_notification_heights(NOTIFICATION_WIDTH)
                    .iter(),
            )
            .enumerate()
        {
            let notification_rect = Rect {
                x: area.x + area.width.saturating_sub(NOTIFICATION_WIDTH),
                y: area.y + (i as u16 * NOTIFICATION_HEIGHT),
                width: NOTIFICATION_WIDTH,
                height: *height,
            };
            frame.render_widget(notification_widget.clone(), notification_rect);
        }
    }
}

fn render_terminal(frame: &mut Frame, app: &mut App, inner_area: Rect) {
    let terminal_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(inner_area);

    let history_area = terminal_chunks[0];
    let input_area = terminal_chunks[1];

    // Display Log
    let log_lines: Vec<Line> = app
        .sql_terminal
        .displayed_lines
        .iter()
        .map(|line| Line::from(line.clone()))
        .collect();

    let log_paragraph = Paragraph::new(log_lines.clone())
        .block(Block::default().padding(Padding::horizontal(1)))
        .fg(app.theme.body_text)
        .wrap(ratatui::widgets::Wrap { trim: true });

    // Auto-scroll to bottom
    let scroll = (log_lines.len() as u16).saturating_sub(history_area.height);
    let log_paragraph = log_paragraph.scroll((scroll, 0));

    frame.render_widget(log_paragraph, history_area);

    // Input
    let input_text = &app.sql_terminal.input;
    let visible_width = input_area.width.saturating_sub(2); // inside borders

    let cursor_offset_in_para = (2 + app.sql_terminal.cursor_index) as u16;
    let scroll_x = cursor_offset_in_para.saturating_sub(visible_width);

    let input_paragraph = Paragraph::new(format!("> {}", input_text))
        .fg(app.theme.header_text)
        .block(
            Block::bordered()
                .border_style(Style::default().fg(app.theme.inner_border))
                .border_set(border::ROUNDED),
        )
        .scroll((0, scroll_x));

    frame.render_widget(input_paragraph, input_area);

    // Cursor
    frame.set_cursor_position((
        input_area.x + 1 + (cursor_offset_in_para - scroll_x),
        input_area.y + 1,
    ));
}

fn render_results(frame: &mut Frame, app: &mut App, inner_area: Rect) {
    if let Some(table_view) = &mut app.table_view {
        let theme = &app.theme;
        let data = &table_view.data;
        // Adjust Constraint::Length by adding padding
        let constraints: Vec<Constraint> = data
            .max_lengths
            .iter()
            .map(|length| Constraint::Length(*length as u16 + 2))
            .collect();

        let header = Row::new(data.columns.clone())
            .style(
                Style::default()
                    .fg(theme.header_text)
                    .add_modifier(Modifier::BOLD),
            )
            .bottom_margin(1); // Add bottom margin to header row for spacing

        let rows: Vec<Row> = data
            .rows
            .iter()
            .map(|row_data| {
                let cells = row_data
                    .iter()
                    .map(|cell_data| Cell::from(cell_data.as_str()))
                    .collect::<Vec<Cell>>();
                Row::new(cells)
            })
            .collect();

        let table = Table::new(rows, &constraints)
            .header(header)
            .block(Block::default().padding(Padding::horizontal(1)))
            .row_highlight_style(Style::default().bg(theme.highlight).fg(Color::Black))
            .highlight_symbol(">> ");

        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));

        let table_area = inner_area;
        frame.render_stateful_widget(table, table_area, &mut table_view.state);
        frame.render_stateful_widget(
            scrollbar,
            table_area.inner(Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut table_view.scroll_state,
        );
    }
}

fn render_help(frame: &mut Frame, app: &mut App) {
    let floating_window_rect = floating_window(frame, &app.theme);
    let commands = Paragraph::new("HELP")
        .centered()
        .block(Block::default().padding(Padding::uniform(2)))
        .fg(app.theme.body_text);

    frame.render_widget(commands, floating_window_rect);
}

fn render_exiting(frame: &mut Frame, app: &mut App) {
    let floating_window_rect = floating_window(frame, &app.theme);

    let confirmation = Paragraph::new(format!("Quit {} Session? y/n", TOOL_NAME))
        .centered()
        .block(Block::default().padding(Padding::uniform(2)))
        .bold()
        .fg(app.theme.body_text);

    frame.render_widget(confirmation, floating_window_rect);
}
