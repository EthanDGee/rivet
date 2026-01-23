const MAX_HISTORY_LENGTH: usize = 100;

pub struct SqlTerminal {
    history: Vec<String>,
    history_index: usize,
    pub input: String,
    cursor_index: usize,
}

impl SqlTerminal {
    pub fn new() -> Self {
        let mut history: Vec<String> = vec!["".to_string()];
        history.push("".to_string());
        SqlTerminal {
            history,
            history_index: 0,
            input: "".to_string(),
            cursor_index: 0,
        }
    }

    // input operations
    pub fn move_cursor_left(&mut self) {
        if self.cursor_index == 0 {
            return;
        }
        self.cursor_index -= 1
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_index == self.input.len() {
            return;
        }
        self.cursor_index += 1
    }

    pub fn enter_char(&mut self, new_char: char) {
        self.input.insert(self.cursor_index, new_char);
        self.move_cursor_right();
    }

    pub fn delete_char(&mut self) {
        if self.cursor_index == 0 {
            return;
        }

        // take left and right half skipping cursor_index and merge back together
        let left_half = self.input.chars().take(self.cursor_index);
        let right_half = self.input.chars().skip(self.cursor_index);
        self.input = left_half.chain(right_half).collect();

        self.move_cursor_left();
    }

    // history operations
    pub fn increment_history(&mut self) {
        if self.history_index == self.history.len() - 1 {
            return;
        }

        // save the current_state and swap input to next
        self.history[self.history_index] = self.input.clone();
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

    pub fn add_command(&mut self) {
        let mut len_history = self.history.len();

        // modify history to move command to last executed
        if self.history_index != len_history - 1 {
            self.history[len_history - 1] = self.input.clone();
        }

        // trim history if over length
        if len_history >= MAX_HISTORY_LENGTH {
            let extra_elements: usize = len_history - MAX_HISTORY_LENGTH;
            self.history.drain(0..extra_elements);
            len_history = self.history.len()
        }
        // update index to latest and add new blank
        self.history_index = len_history - 1;
        self.history.push("".to_string());
        self.cursor_index = 0;
    }
}
