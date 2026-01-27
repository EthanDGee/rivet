use std::collections::VecDeque;
const MAX_HISTORY_LENGTH: usize = 100;
const MAX_LOG_LINES: usize = 1000;
const MAX_INPUT_LENGTH: usize = 2048;

pub struct SqlTerminal {
    pub history: Vec<String>,
    history_index: usize,
    pub input: String,
    cursor_index: usize,
    pub displayed_lines: VecDeque<String>,
}

impl SqlTerminal {
    pub fn new() -> Self {
        let history: Vec<String> = Vec::with_capacity(1);
        SqlTerminal {
            history,
            history_index: 0,
            input: String::new(),
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
        if self.cursor_index >= self.input.chars().count() {
            return;
        }
        self.cursor_index += 1;
    }

    pub fn enter_char(&mut self, new_char: char) {
        if self.input.chars().count() >= MAX_INPUT_LENGTH {
            return;
        }

        self.input.insert(self.cursor_index, new_char);
        self.move_cursor_right();
    }

    pub fn delete_char(&mut self) {
        if self.cursor_index == 0 {
            return;
        }
        let mut chars: Vec<char> = self.input.chars().collect();
        if self.cursor_index <= chars.len() {
            chars.remove(self.cursor_index - 1);
            self.input = chars.into_iter().collect();
            self.cursor_index -= 1;
        }
    }

    // history operations
    pub fn increment_history(&mut self) {
        if self.history.is_empty() || self.history_index >= self.history.len().saturating_sub(1) {
            return;
        }
        // Save current state and navigate
        self.history[self.history_index] = self.input.clone();
        self.history_index += 1;
        self.input = self.history[self.history_index].clone();
        self.cursor_index = self.input.chars().count();
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
            self.input = String::new();
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
        self.history.push(String::new());
        self.input = String::new();
        self.cursor_index = 0;
    }
}
