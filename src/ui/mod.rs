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
