use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Table {
    pub rows: Vec<Vec<String>>,
    pub has_header: bool,
}

impl Table {
    pub fn new(rows: Vec<Vec<String>>) -> Self {
        let has_header = rows.len() >= 2 && is_separator_row(&rows[1]);
        Self { rows, has_header }
    }
    
    pub fn new_with_header(rows: Vec<Vec<String>>, has_header: bool) -> Self {
        Self { rows, has_header }
    }
}

fn is_separator_row(row: &Vec<String>) -> bool {
    row.iter().all(|cell| {
        let trimmed = cell.trim();
        trimmed.chars().all(|c| c == '-' || c == ':') && trimmed.contains('-')
    })
}
