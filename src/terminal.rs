use std::collections::VecDeque;
const MAX_HISTORY_LENGTH: usize = 100;
const MAX_LOG_LINES: usize = 1000;

pub struct SqlTerminal {
    pub history: Vec<String>,
    history_index: usize,
    pub input: String,
    cursor_index: usize,
    pub displayed_lines: VecDeque<String>,
}

impl SqlTerminal {
    pub fn new() -> Self {
        let history: Vec<String> = vec!["".to_string()];
        SqlTerminal {
            history,
            history_index: 0,
            input: "".to_string(),
            cursor_index: 0,
            displayed_lines: VecDeque::with_capacity(MAX_LOG_LINES),
        }
    }

    pub fn add_log_line(&mut self, line: String) {
        if self.displayed_lines.len() == MAX_LOG_LINES {
            self.displayed_lines.pop_front();
        }
        self.displayed_lines.push_back(line);
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
        // Find the byte index of the char before the cursor
        let byte_idx = self
            .input
            .char_indices()
            .nth(self.cursor_index - 1)
            .map(|(idx, _)| idx)
            .unwrap();
        self.input.remove(byte_idx);
        self.cursor_index -= 1;
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
        // save the current and swap input to the previous
        self.history[self.history_index] = self.input.clone();
        self.history_index -= 1;
        self.input = self.history[self.history_index].clone();

        // move cursor to end of line
        self.cursor_index = self.input.len();
    }

    pub fn add_command(&mut self) {
        self.input = self.input.trim_end().to_string();

        if self.input.is_empty() {
            return;
        }

        // ensure duplicate entries are not added alongside each other
        if self.history.last().is_some_and(|x| *x == self.input) {
            self.input = "".to_string();
            return;
        }

        let mut len_history = self.history.len();

        // replace first in history if empty
        if len_history == 1 {
            self.history[0] = self.input.clone();
        } else
        // else modify history to move command to last executed
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
        self.input = "".to_string();
        self.cursor_index = 0;
    }
}
