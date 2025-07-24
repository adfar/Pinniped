use crate::block::Block;
use crate::document::Document;
use crate::error::Error;
use crate::table::Table;
use crate::inline::{parse_inline, InlineText};

/// Parse markdown text into a Document.
pub fn parse(input: &str) -> Result<Document, Error> {
    let mut blocks = Vec::new();
    let mut chars = input.chars().peekable();
    let mut current_section = String::new();
    let mut in_code_block = false;
    let mut code_block_lang: Option<String> = None;
    let mut code_block_content = String::new();
    
    while let Some(ch) = chars.next() {
        if ch == '`' && chars.peek() == Some(&'`') {
            chars.next(); // consume second `
            if chars.peek() == Some(&'`') {
                chars.next(); // consume third `
                if in_code_block {
                    // End of code block
                    blocks.push(Block::CodeBlock { 
                        language: code_block_lang.clone(), 
                        code: code_block_content.clone() 
                    });
                    code_block_content.clear();
                    code_block_lang = None;
                    in_code_block = false;
                    // Skip to end of line
                    while let Some(ch) = chars.peek() {
                        if *ch == '\n' {
                            chars.next();
                            break;
                        }
                        chars.next();
                    }
                } else {
                    // Start of code block - parse language
                    if !current_section.trim().is_empty() {
                        parse_section(&current_section.trim(), &mut blocks);
                        current_section.clear();
                    }
                    let mut lang = String::new();
                    while let Some(ch) = chars.peek() {
                        if *ch == '\n' {
                            chars.next();
                            break;
                        }
                        lang.push(chars.next().unwrap());
                    }
                    code_block_lang = if lang.trim().is_empty() { None } else { Some(lang.trim().to_string()) };
                    in_code_block = true;
                }
            } else {
                current_section.push('`');
                current_section.push('`');
            }
        } else if in_code_block {
            code_block_content.push(ch);
        } else {
            current_section.push(ch);
        }
    }
    
    if in_code_block {
        blocks.push(Block::CodeBlock { 
            language: code_block_lang, 
            code: code_block_content 
        });
    } else if !current_section.trim().is_empty() {
        parse_section(&current_section.trim(), &mut blocks);
    }
    
    Ok(Document::new(blocks))
}

fn parse_section(section: &str, blocks: &mut Vec<Block>) {
    let sections = section.split("\n\n");
    for section in sections {
        let trimmed = section.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.starts_with('#') {
            let level = trimmed.chars().take_while(|&c| c == '#').count() as u8;
            let text = trimmed.chars().skip(level as usize).collect::<String>().trim().to_string();
            blocks.push(Block::Header { level, text: parse_inline(&text) });
        } else if trimmed.lines().all(|line| line.trim().starts_with("> ")) {
            let quote_text = trimmed
                .lines()
                .map(|line| line.trim().strip_prefix("> ").unwrap_or(line))
                .collect::<Vec<&str>>()
                .join("\n");
            blocks.push(Block::Blockquote(parse_inline(&quote_text)));
        } else if trimmed.lines().all(|line| line.trim().starts_with("- ")) {
            let items: Vec<InlineText> = trimmed
                .lines()
                .map(|line| parse_inline(line.trim().strip_prefix("- ").unwrap_or(line)))
                .collect();
            blocks.push(Block::UnorderedList(items));
        } else if trimmed.lines().all(|line| {
            let trimmed_line = line.trim();
            trimmed_line.chars().next().map_or(false, |c| c.is_ascii_digit()) &&
            trimmed_line.contains(". ")
        }) {
            let items: Vec<InlineText> = trimmed
                .lines()
                .map(|line| {
                    let trimmed_line = line.trim();
                    if let Some(dot_pos) = trimmed_line.find(". ") {
                        parse_inline(&trimmed_line[dot_pos + 2..])
                    } else {
                        parse_inline(trimmed_line)
                    }
                })
                .collect();
            blocks.push(Block::OrderedList(items));
        } else if trimmed.contains('|') && trimmed.lines().all(|l| l.contains('|')) {
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
            blocks.push(Block::Paragraph(parse_inline(trimmed)));
        }
    }
}

/// Convert a Document back into markdown text.
pub fn to_markdown(doc: &Document) -> String {
    let parts: Vec<String> = doc
        .blocks
        .iter()
        .map(|block| match block {
            Block::Paragraph(text) => text.to_markdown(),
            Block::Header { level, text } => format!("{} {}", "#".repeat(*level as usize), text.to_markdown()),
            Block::UnorderedList(items) => items
                .iter()
                .map(|item| format!("- {}", item.to_markdown()))
                .collect::<Vec<String>>()
                .join("\n"),
            Block::OrderedList(items) => items
                .iter()
                .enumerate()
                .map(|(i, item)| format!("{}. {}", i + 1, item.to_markdown()))
                .collect::<Vec<String>>()
                .join("\n"),
            Block::CodeBlock { language, code } => {
                match language {
                    Some(lang) => format!("```{}\n{}\n```", lang, code.trim_end()),
                    None => format!("```\n{}\n```", code.trim_end()),
                }
            },
            Block::Blockquote(text) => {
                text.to_markdown()
                    .lines()
                    .map(|line| format!("> {}", line))
                    .collect::<Vec<String>>()
                    .join("\n")
            },
            Block::Table(table) => {
                if table.has_header && table.rows.len() >= 2 {
                    let mut result = Vec::new();
                    result.push(format!("|{}|", table.rows[0].join("|")));
                    result.push(format!("|{}|", table.rows[1].join("|")));
                    for row in &table.rows[2..] {
                        result.push(format!("|{}|", row.join("|")));
                    }
                    result.join("\n")
                } else {
                    table
                        .rows
                        .iter()
                        .map(|row| format!("|{}|", row.join("|")))
                        .collect::<Vec<String>>()
                        .join("\n")
                }
            },
        })
        .collect();
    parts.join("\n\n")
}
