use super::themes::ColorPalette;
use crate::model::notifications::Notification;
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::{Block, Padding, Paragraph};

pub const NOTIFICATION_WIDTH: u16 = 32;
pub const NOTIFICATION_HEIGHT: u16 = 5;

pub fn render_notification<'a>(notification: &'a Notification, theme: &'a ColorPalette) -> Paragraph<'a> {
    return Paragraph::new(Line::from(notification.message.clone()))
        .block(
            Block::bordered()
                .title(Line::from(notification.title.clone()).centered())
                .padding(Padding::uniform(1))
                .border_style(Style::default().fg(theme.inner_border)),
        )
        .style(Style::default().fg(theme.body_text).bg(theme.background))
        .wrap(ratatui::widgets::Wrap { trim: false });
}
