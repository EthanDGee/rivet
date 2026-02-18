use color_eyre::Report;
use std::time::{Duration, Instant};

const TIME_LIMIT: Duration = Duration::from_secs(5);

pub struct Notification {
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

    fn new_error(error: Report) -> Self {
        Notification {
            title: "Error".to_string(),
            message: error.to_string(),
            time_stamp: Instant::now(),
        }
    }

    fn expired(&self) -> bool {
        self.time_stamp.elapsed() > TIME_LIMIT
    }
}

pub struct NotificationList {
    pub list: Vec<Notification>,
}

impl NotificationList {
    pub fn new() -> Self {
        NotificationList { list: Vec::new() }
    }

    pub fn notify(&mut self, title: &str, message: &str) {
        self.list.push(Notification::new(title, message))
    }

    pub fn error(&mut self, error: Report) {
        self.list.push(Notification::new_error(error))
    }

    pub fn remove_expired(&mut self) {
        self.list.retain(|notification| !notification.expired());
    }

    pub fn get_notification_heights(&self, width: u16) -> Vec<u16> {
        self.list
            .iter()
            .map(|notification| {
                let inner_width = width.saturating_sub(2);
                let char_count = notification.message.chars().count();
                let wrapped_lines = (char_count / inner_width as usize) as u16 + 1;
                wrapped_lines + 4
            })
            .collect()
    }
}
