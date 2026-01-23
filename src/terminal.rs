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
}
