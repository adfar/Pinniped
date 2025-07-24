use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InlineElement {
    Text(String),
    Bold(Vec<InlineElement>),
    Italic(Vec<InlineElement>),
    Code(String),
    Link { text: String, url: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineText {
    pub elements: Vec<InlineElement>,
}

impl InlineText {
    pub fn new(elements: Vec<InlineElement>) -> Self {
        Self { elements }
    }
    
    pub fn from_plain(text: String) -> Self {
        Self {
            elements: vec![InlineElement::Text(text)],
        }
    }
    
    pub fn to_markdown(&self) -> String {
        self.elements.iter().map(|element| element.to_markdown()).collect()
    }
}

impl InlineElement {
    pub fn to_markdown(&self) -> String {
        match self {
            InlineElement::Text(text) => text.clone(),
            InlineElement::Bold(elements) => {
                let content = elements.iter().map(|e| e.to_markdown()).collect::<String>();
                format!("**{}**", content)
            },
            InlineElement::Italic(elements) => {
                let content = elements.iter().map(|e| e.to_markdown()).collect::<String>();
                format!("*{}*", content)
            },
            InlineElement::Code(text) => format!("`{}`", text),
            InlineElement::Link { text, url } => format!("[{}]({})", text, url),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Text(String),
    Bold,        // **
    Italic,      // *
    Code,        // `
    LinkStart,   // [
    LinkMiddle,  // ](
    LinkEnd,     // )
}

pub fn parse_inline(input: &str) -> InlineText {
    let tokens = tokenize(input);
    let elements = parse_tokens(&tokens);
    InlineText::new(elements)
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    
    while i < chars.len() {
        match chars[i] {
            '*' => {
                // Determine if this should be ** or *
                if i + 2 < chars.len() && chars[i + 1] == '*' && chars[i + 2] == '*' {
                    // Triple star case - need to look ahead to decide
                    let (first_token, second_token) = decide_triple_star(&chars, i);
                    tokens.push(first_token);
                    if let Some(token) = second_token {
                        tokens.push(token);
                    }
                    i += 3;
                } else if i + 1 < chars.len() && chars[i + 1] == '*' {
                    tokens.push(Token::Bold);
                    i += 2;
                } else {
                    tokens.push(Token::Italic);
                    i += 1;
                }
            },
            '`' => {
                tokens.push(Token::Code);
                i += 1;
            },
            '[' => {
                tokens.push(Token::LinkStart);
                i += 1;
            },
            ']' => {
                if i + 1 < chars.len() && chars[i + 1] == '(' {
                    tokens.push(Token::LinkMiddle);
                    i += 2;
                } else {
                    // Not a link, treat as text
                    if let Some(Token::Text(ref mut text)) = tokens.last_mut() {
                        text.push(']');
                    } else {
                        tokens.push(Token::Text("]".to_string()));
                    }
                    i += 1;
                }
            },
            ')' => {
                tokens.push(Token::LinkEnd);
                i += 1;
            },
            _ => {
                // Collect consecutive text characters
                let mut text = String::new();
                while i < chars.len() && !matches!(chars[i], '*' | '`' | '[' | ']' | ')') {
                    text.push(chars[i]);
                    i += 1;
                }
                if !text.is_empty() {
                    tokens.push(Token::Text(text));
                }
            }
        }
    }
    
    tokens
}

fn decide_triple_star(chars: &[char], pos: usize) -> (Token, Option<Token>) {
    let full_text: String = chars[pos..].iter().collect();
    
    // Simulate both interpretations and see which balances better
    let bold_first_score = simulate_balance(&full_text, true); // *** as ** + *
    let italic_first_score = simulate_balance(&full_text, false); // *** as * + **
    
    if bold_first_score <= italic_first_score {
        (Token::Bold, Some(Token::Italic)) // ** + *
    } else {
        (Token::Italic, Some(Token::Bold)) // * + **
    }
}

fn simulate_balance(text: &str, bold_first: bool) -> i32 {
    #[derive(Debug, Clone, PartialEq)]
    enum DelimiterType {
        Bold,
        Italic,
    }
    
    let mut stack: Vec<DelimiterType> = Vec::new();
    let mut penalty = 0;
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;
    
    // Handle the initial ***
    if bold_first {
        stack.push(DelimiterType::Bold);   // **
        stack.push(DelimiterType::Italic); // *
        i += 3;
    } else {
        stack.push(DelimiterType::Italic); // *
        stack.push(DelimiterType::Bold);   // **
        i += 3;
    }
    
    // Process remaining tokens
    while i < chars.len() {
        if i + 1 < chars.len() && chars[i] == '*' && chars[i + 1] == '*' {
            // Found ** - try to match with Bold
            if let Some(pos) = stack.iter().rposition(|d| *d == DelimiterType::Bold) {
                // Found a Bold delimiter to match
                let distance_from_top = stack.len() - 1 - pos;
                if distance_from_top > 0 {
                    // Not the top delimiter - add penalty for unnatural nesting
                    penalty += distance_from_top as i32 * 3;
                }
                stack.remove(pos);
            } else {
                // No matching Bold delimiter - unmatched closing
                penalty += 15;
            }
            i += 2;
        } else if chars[i] == '*' {
            // Found * - try to match with Italic
            if let Some(pos) = stack.iter().rposition(|d| *d == DelimiterType::Italic) {
                // Found an Italic delimiter to match
                let distance_from_top = stack.len() - 1 - pos;
                if distance_from_top > 0 {
                    // Not the top delimiter - add penalty for unnatural nesting
                    penalty += distance_from_top as i32 * 3;
                }
                stack.remove(pos);
            } else {
                // No matching Italic delimiter - unmatched closing
                penalty += 15;
            }
            i += 1;
        } else {
            i += 1;
        }
    }
    
    // Penalty for unmatched opening delimiters
    penalty + stack.len() as i32 * 8
}

// Debug version that can be called from tests
pub fn simulate_balance_debug(text: &str, bold_first: bool) -> i32 {
    simulate_balance(text, bold_first)
}

fn parse_tokens(tokens: &[Token]) -> Vec<InlineElement> {
    let mut elements = Vec::new();
    let mut i = 0;
    
    while i < tokens.len() {
        match &tokens[i] {
            Token::Text(text) => {
                elements.push(InlineElement::Text(text.clone()));
                i += 1;
            },
            Token::Bold => {
                if let Some((content, consumed)) = parse_delimited(tokens, i, Token::Bold) {
                    elements.push(InlineElement::Bold(content));
                    i += consumed;
                } else {
                    elements.push(InlineElement::Text("**".to_string()));
                    i += 1;
                }
            },
            Token::Italic => {
                if let Some((content, consumed)) = parse_delimited(tokens, i, Token::Italic) {
                    elements.push(InlineElement::Italic(content));
                    i += consumed;
                } else {
                    elements.push(InlineElement::Text("*".to_string()));
                    i += 1;
                }
            },
            Token::Code => {
                if let Some((text, consumed)) = parse_code(tokens, i) {
                    elements.push(InlineElement::Code(text));
                    i += consumed;
                } else {
                    elements.push(InlineElement::Text("`".to_string()));
                    i += 1;
                }
            },
            Token::LinkStart => {
                if let Some((link, consumed)) = parse_link(tokens, i) {
                    elements.push(link);
                    i += consumed;
                } else {
                    elements.push(InlineElement::Text("[".to_string()));
                    i += 1;
                }
            },
            _ => {
                // Unexpected token, treat as text
                elements.push(InlineElement::Text("?".to_string()));
                i += 1;
            }
        }
    }
    
    elements
}

fn parse_delimited(tokens: &[Token], start: usize, delimiter: Token) -> Option<(Vec<InlineElement>, usize)> {
    let mut i = start + 1; // Skip opening delimiter
    let mut nested_tokens = Vec::new();
    
    while i < tokens.len() {
        if tokens[i] == delimiter {
            // Found closing delimiter
            let content = parse_tokens(&nested_tokens);
            return Some((content, i - start + 1));
        } else {
            nested_tokens.push(tokens[i].clone());
            i += 1;
        }
    }
    
    None // No closing delimiter found
}

fn parse_code(tokens: &[Token], start: usize) -> Option<(String, usize)> {
    let mut i = start + 1; // Skip opening `
    let mut text = String::new();
    
    while i < tokens.len() {
        match &tokens[i] {
            Token::Code => {
                // Found closing `
                return Some((text, i - start + 1));
            },
            Token::Text(t) => text.push_str(t),
            Token::Bold => text.push_str("**"),
            Token::Italic => text.push('*'),
            Token::LinkStart => text.push('['),
            Token::LinkMiddle => text.push_str("]("),
            Token::LinkEnd => text.push(')'),
        }
        i += 1;
    }
    
    None // No closing ` found
}

fn parse_link(tokens: &[Token], start: usize) -> Option<(InlineElement, usize)> {
    let mut i = start + 1; // Skip [
    let mut link_text = String::new();
    
    // Parse link text
    while i < tokens.len() {
        match &tokens[i] {
            Token::LinkMiddle => {
                i += 1; // Skip ](
                break;
            },
            Token::Text(t) => link_text.push_str(t),
            _ => {
                i += 1;
            }
        }
        i += 1;
    }
    
    // Parse URL
    let mut url = String::new();
    while i < tokens.len() {
        match &tokens[i] {
            Token::LinkEnd => {
                // Found closing )
                return Some((InlineElement::Link { text: link_text, url }, i - start + 1));
            },
            Token::Text(t) => url.push_str(t),
            _ => {
                i += 1;
            }
        }
        i += 1;
    }
    
    None // Malformed link
}