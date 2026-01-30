use std::time::{Duration, Instant};
const TIME_LIMIT: Duration = Duration::from_secs(5);

struct Notification {
    pub title: String,
    pub message: String,
    time_stamp: Instant,
}

impl Notification {
    fn new(title: &String, message: &String) -> Self {
        Notification {
            title: title.clone(),
            message: message.clone(),
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

    pub fn notify(&mut self, title: &String, message: &String) {
        self.list.push(Notification::new(title, message))
    }

    pub fn remove_expired(&mut self) {
        self.list.retain(|notification| !notification.expired());
    }
}
