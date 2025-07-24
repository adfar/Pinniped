#[derive(Debug, Clone, PartialEq)]
pub struct Table {
    pub rows: Vec<Vec<String>>, 
}

impl Table {
    pub fn new(rows: Vec<Vec<String>>) -> Self {
        Self { rows }
    }
}
