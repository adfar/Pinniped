use crate::block::Block;
use crate::error::Error;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Document {
    pub blocks: Vec<Block>,
}

impl Document {
    /// Create a new Document from blocks.
    pub fn new(blocks: Vec<Block>) -> Self {
        Self { blocks }
    }

    /// Parse markdown text into a Document.
    pub fn parse(markdown: &str) -> Result<Self, Error> {
        crate::parser::parse(markdown)
    }

    /// Convert the Document back to markdown text.
    pub fn to_markdown(&self) -> String {
        crate::parser::to_markdown(self)
    }
}
