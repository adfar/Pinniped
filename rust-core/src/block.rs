use crate::table::Table;

#[derive(Debug, Clone, PartialEq)]
pub enum Block {
    Paragraph(String),
    Table(Table),
}
