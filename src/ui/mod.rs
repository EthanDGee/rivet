pub mod notifications;
pub mod table;
pub mod terminal;
pub mod themes;
pub mod utils;

use crate::app::App;
use crate::app::TOOL_NAME;
use crate::screens::Screen;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, Cell, Padding, Paragraph, Row, Scrollbar, ScrollbarOrientation, Table},
};
use std::format;
use std::vec;
use utils::floating_window;

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
