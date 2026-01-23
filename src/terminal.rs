const MAX_HISTORY_LENGTH: u8 = 100;

struct Terminal {
    history: Vec<String>,
    history_index: usize,
    input: String,
    cursor_index: usize,
}

impl Terminal {
    pub fn new() -> Self {
        let mut history: Vec<String> = Vec::new();
        history.push("".to_string());
        Terminal {
            history,
            history_index: 0,
            input: "".to_string(),
            cursor_index: 0,
        }
    }

    pub fn increment_history(&mut self) {
        if self.history_index == self.history.len() - 1 {
            return;
        }

        // save the current_state and swap input to next        self.history[self.history_index] = self.input.clone();
        self.history_index += 1;
        self.input = self.history[self.history_index].clone();

        // move cursor to end of line
        self.cursor_index = self.input.len();
    }

    pub fn decrement_history(&mut self) {
        if self.history_index == 0 {
            return;
        }
        // save the current and swap input to previous
        self.history[self.history_index] = self.input.clone();
        self.history_index -= 1;
        self.input = self.history[self.history_index].clone();

        // move cursor to end of line
        self.cursor_index = self.input.len();
    }
}
