use super::themes::ColorPalette;
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::{Block, Padding, Paragraph};
use std::time::{Duration, Instant};

const TIME_LIMIT: Duration = Duration::from_secs(5);

struct Notification {
    pub title: String,
    pub message: String,
    time_stamp: Instant,
}

impl Notification {
    fn new(title: &str, message: &str) -> Self {
        Notification {
            title: title.to_string(),
            message: message.to_string(),
            time_stamp: Instant::now(),
        }
    }

    fn expired(&self) -> bool {
        self.time_stamp.elapsed() > TIME_LIMIT
    }
}

pub struct NotificationList {
    list: Vec<Notification>,
}

impl NotificationList {
    pub fn new() -> Self {
        NotificationList { list: Vec::new() }
    }

    pub fn notify(&mut self, title: &str, message: &str) {
        self.list.push(Notification::new(title, message))
    }

    pub fn remove_expired(&mut self) {
        self.list.retain(|notification| !notification.expired());
    }

    pub fn get_notification_widgets(&self, theme: &ColorPalette) -> Vec<Paragraph> {
        self.list
            .iter()
            .map(|notification| {
                Paragraph::new(Line::from(notification.message.clone()))
                    .block(
                        Block::bordered()
                            .title(Line::from(notification.title.clone()).centered())
                            .padding(Padding::uniform(1)) // Add padding to ensure text doesn't touch border
                            .border_style(Style::default().fg(theme.inner_border)),
                    )
                    .style(Style::default().fg(theme.body_text)) // Removed explicit background color
                    .wrap(ratatui::widgets::Wrap { trim: false }) // Enable text wrapping
            })
            .collect()
    }
}
