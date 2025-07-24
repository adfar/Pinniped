use crate::table::Table;
use crate::inline::InlineText;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Block {
    Paragraph(InlineText),
    Header { level: u8, text: InlineText },
    UnorderedList(Vec<InlineText>),
    OrderedList(Vec<InlineText>),
    CodeBlock { language: Option<String>, code: String },
    Blockquote(InlineText),
    Table(Table),
}
