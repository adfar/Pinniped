use crate::block::Block;
use crate::document::Document;
use crate::error::Error;
use crate::table::Table;

/// Parse markdown text into a Document.
pub fn parse(input: &str) -> Result<Document, Error> {
    let mut blocks = Vec::new();
    let sections = input.split("\n\n");
    for section in sections {
        let trimmed = section.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.contains('|') && trimmed.lines().all(|l| l.contains('|')) {
            let rows = trimmed
                .lines()
                .map(|line| {
                    line.trim().trim_matches('|')
                        .split('|')
                        .map(|cell| cell.trim().to_string())
                        .collect::<Vec<String>>()
                })
                .collect::<Vec<Vec<String>>>();
            blocks.push(Block::Table(Table::new(rows)));
        } else {
            blocks.push(Block::Paragraph(trimmed.to_string()));
        }
    }
    Ok(Document::new(blocks))
}

/// Convert a Document back into markdown text.
pub fn to_markdown(doc: &Document) -> String {
    let parts: Vec<String> = doc
        .blocks
        .iter()
        .map(|block| match block {
            Block::Paragraph(text) => text.clone(),
            Block::Table(table) => table
                .rows
                .iter()
                .map(|row| format!("|{}|", row.join("|")))
                .collect::<Vec<String>>()
                .join("\n"),
        })
        .collect();
    parts.join("\n\n")
}
