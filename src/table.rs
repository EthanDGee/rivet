use ratatui::widgets::{ScrollbarState, TableState};
use std::vec;

const ITEM_HEIGHT: usize = 4;

struct TableData {
    columns: Vec<String>,
    max_lengths: Vec<u8>,
    rows: Vec<Vec<String>>,
}

impl TableData {
    pub fn new(columns: Vec<String>, rows: Vec<Vec<String>>) -> Self {
        // calculate the max_lengths for each columns

        let mut max_lengths: Vec<u8> = vec![];

        for col in 0..(columns.len() - 1) {
            // set the max lengths initially to be the lengths of the column names
            max_lengths.push(columns[col].as_str().len() as u8);

            // get max length for each column
            let max_row_length: u8 = rows[col]
                .iter()
                .map(|s| s.as_str().len())
                .max()
                .unwrap_or(0) as u8;

            if max_lengths[col] < max_row_length {
                max_lengths[col] = max_row_length;
            }
        }

        TableData {
            columns,
            max_lengths,
            rows,
        }
    }
}

pub struct TableView {
    data: TableData,
    state: TableState,
    scroll_state: ScrollbarState,
}

impl TableView {
    pub fn new(columns: Vec<String>, rows: Vec<Vec<String>>) -> Self {
        let state = TableState::default().with_offset(0);
        let scroll_state: ScrollbarState = ScrollbarState::new((rows.len() - 1) * ITEM_HEIGHT);
        TableView {
            data: TableData::new(columns, rows),
            state,
            scroll_state,
        }
    }

    // Add navigation operations
    pub fn next_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.data.rows.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn previous_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.data.rows.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn next_column(&mut self) {
        self.state.select_next_column();
    }

    pub fn previous_column(&mut self) {
        self.state.select_previous_column();
    }
}
