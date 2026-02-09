use ratatui::widgets::{ScrollbarState, TableState};

const ITEM_HEIGHT: usize = 4;

#[derive(Debug)]
pub struct TableData {
    pub columns: Vec<String>,
    pub max_lengths: Vec<u8>,
    pub rows: Vec<Vec<String>>,
}

impl TableData {
    pub fn new(columns: Vec<String>, rows: Vec<Vec<String>>) -> Self {
        // calculate the max_lengths for each columns
        let mut max_lengths: Vec<u8> = columns.iter().map(|s| s.len() as u8).collect();

        if !rows.is_empty() {
            for i in 0..columns.len() {
                let max_in_col = rows
                    .iter()
                    .map(|row| row.get(i).map(|cell| cell.len()).unwrap_or(0))
                    .max()
                    .unwrap_or(0) as u8;

                if i < max_lengths.len() {
                    max_lengths[i] = max_lengths[i].max(max_in_col);
                } else {
                    max_lengths.push(max_in_col);
                }
            }
        }

        TableData {
            columns,
            max_lengths,
            rows,
        }
    }
}

#[derive(Debug)]
pub struct TableView {
    pub data: TableData,
    pub state: TableState,
    pub scroll_state: ScrollbarState,
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
