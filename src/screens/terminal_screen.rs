use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Modifier, Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, Padding, Paragraph},
};
use std::collections::VecDeque;

use crate::app::{App, TOOL_NAME};
use crate::ui::themes::ColorPalette;

const MAX_HISTORY_LENGTH: usize = 100;
const MAX_LOG_LINES: usize = 1000;
const MAX_INPUT_LENGTH: usize = 2048;

#[derive(Debug, Default)]
pub struct TerminalScreen {
    pub history: Vec<String>,
    history_index: usize,
    pub input: String,
    pub cursor_index: usize,
    pub displayed_lines: VecDeque<String>,
    // Cache to reduce allocations during history navigation
    input_backup: Option<String>,
}

impl TerminalScreen {
    pub fn new() -> Self {
        let history: Vec<String> = vec![String::new()];
        TerminalScreen {
            history,
            history_index: 0,
            input: String::new(),
            cursor_index: 0,
            displayed_lines: VecDeque::with_capacity(MAX_LOG_LINES),
            input_backup: None,
        }
    }

    pub fn render(&self, frame: &mut Frame, app: &App, inner_area: Rect) {
        let terminal_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(inner_area);

        let history_area = terminal_chunks[0];
        let input_area = terminal_chunks[1];

        // Display Log
        let log_lines: Vec<Line> = self
            .displayed_lines
            .iter()
            .map(|line| Line::from(line.clone()))
            .collect();

        let log_paragraph = Paragraph::new(log_lines.clone())
            .block(Block::default().padding(Padding::horizontal(1)))
            .fg(app.theme.body_text)
            .wrap(ratatui::widgets::Wrap { trim: true });

        // Auto-scroll to bottom
        let scroll = (log_lines.len() as u16).saturating_sub(history_area.height);
        let log_paragraph = log_paragraph.scroll((scroll, 0));

        frame.render_widget(log_paragraph, history_area);

        // Input
        let input_text = &self.input;
        let visible_width = input_area.width.saturating_sub(2); // inside borders

        let cursor_offset_in_para = (2 + self.cursor_index) as u16;
        let scroll_x = cursor_offset_in_para.saturating_sub(visible_width);

        let input_paragraph = Paragraph::new(format!("> {}", input_text))
            .fg(app.theme.header_text)
            .block(
                Block::bordered()
                    .border_style(Style::default().fg(app.theme.inner_border))
                    .border_set(border::ROUNDED),
            )
            .scroll((0, scroll_x));

        frame.render_widget(input_paragraph, input_area);

        // Cursor
        frame.set_cursor_position((
            input_area.x + 1 + (cursor_offset_in_para - scroll_x),
            input_area.y + 1,
        ));
    }

    pub fn add_log_line(&mut self, line: String) {
        // Only check capacity when approaching limit to reduce overhead
        if self.displayed_lines.len() >= MAX_LOG_LINES {
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

        // Convert to character vector for safe UTF-8 manipulation
        let mut chars: Vec<char> = self.input.chars().collect();
        chars.insert(self.cursor_index, new_char);
        self.input = chars.into_iter().collect();
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

        // Save current input to backup to reduce allocations
        if self.input_backup.is_none() {
            self.input_backup = Some(std::mem::take(&mut self.input));
        } else {
            self.history[self.history_index] = std::mem::take(&mut self.input);
        }

        self.history_index += 1;
        self.input = self.history[self.history_index].clone();
        self.cursor_index = self.input.chars().count();
    }

    pub fn decrement_history(&mut self) {
        if self.history_index == 0 {
            return;
        }

        // Save current input using swap to reduce allocations
        if self.input_backup.is_none() {
            self.input_backup = Some(std::mem::take(&mut self.input));
        } else {
            self.history[self.history_index] = std::mem::take(&mut self.input);
        }

        self.history_index -= 1;
        self.input = self.history[self.history_index].clone();

        // move cursor to end of line (character-based, not byte-based)
        self.cursor_index = self.input.chars().count();
    }

    pub fn add_command(&mut self) {
        // Trim in-place for efficiency
        let new_len = self.input.trim_end().len();
        self.input.truncate(new_len);

        if self.input.is_empty() {
            self.input.clear();
            return;
        }

        // ensure duplicate entries are not added alongside each other
        if self.history.last().is_some_and(|x| *x == self.input) {
            self.input.clear();
            return;
        }

        let mut len_history = self.history.len();

        // replace first in history if empty
        if len_history == 1 {
            self.history[0] = std::mem::take(&mut self.input);
        } else
        // else modify history to move command to last executed
        if self.history_index != len_history - 1 {
            self.history[len_history - 1] = std::mem::take(&mut self.input);
        } else {
            // Take the input and clear it
            std::mem::take(&mut self.input);
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
        self.input.clear(); // Reuse allocated memory instead of creating new String
        self.cursor_index = 0;

        // Clear backup cache
        self.input_backup = None;
    }
}
