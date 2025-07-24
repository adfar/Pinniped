use pinniped_core::document::Document;

#[test]
fn round_trip_paragraph_and_table() {
    let markdown = "Hello world\n\n|a|b|\n|c|d|";
    let doc = Document::parse(markdown).expect("parse");
    let output = doc.to_markdown();
    assert_eq!(markdown, output);
}

#[test]
fn headers() {
    let markdown = "# Header 1\n\n## Header 2\n\n### Header 3";
    let doc = Document::parse(markdown).expect("parse");
    let output = doc.to_markdown();
    assert_eq!(markdown, output);
}

#[test]
fn headers_with_formatting() {
    let markdown = "# **Bold Header**\n\n## *Italic Header*\n\n### `Code Header`";
    let doc = Document::parse(markdown).expect("parse");
    let output = doc.to_markdown();
    assert_eq!(markdown, output);
}

#[test]
fn bold_and_italic() {
    let markdown = "This is **bold** and this is *italic* text.";
    let doc = Document::parse(markdown).expect("parse");
    let output = doc.to_markdown();
    assert_eq!(markdown, output);
}

#[test]
fn inline_code() {
    let markdown = "Use `console.log()` to debug JavaScript.";
    let doc = Document::parse(markdown).expect("parse");
    let output = doc.to_markdown();
    assert_eq!(markdown, output);
}

#[test]
fn mixed_inline_formatting() {
    let markdown = "This has **bold**, *italic*, and `code` all together.";
    let doc = Document::parse(markdown).expect("parse");
    let output = doc.to_markdown();
    assert_eq!(markdown, output);
}

#[test]
fn unordered_lists() {
    let markdown = "- First item\n- Second item\n- Third item";
    let doc = Document::parse(markdown).expect("parse");
    let output = doc.to_markdown();
    assert_eq!(markdown, output);
}

#[test]
fn ordered_lists() {
    let markdown = "1. First item\n2. Second item\n3. Third item";
    let doc = Document::parse(markdown).expect("parse");
    let output = doc.to_markdown();
    assert_eq!(markdown, output);
}

#[test]
fn lists_with_formatting() {
    let markdown = "- **Bold item**\n- *Italic item*\n- `Code item`";
    let doc = Document::parse(markdown).expect("parse");
    let output = doc.to_markdown();
    assert_eq!(markdown, output);
}

#[test]
fn code_blocks() {
    let markdown = "```\nlet x = 42;\nconsole.log(x);\n```";
    let doc = Document::parse(markdown).expect("parse");
    let output = doc.to_markdown();
    assert_eq!(markdown, output);
}

#[test]
fn code_blocks_with_language() {
    let markdown = "```rust\nfn main() {\n    println!(\"Hello, world!\");\n}\n```";
    let doc = Document::parse(markdown).expect("parse");
    let output = doc.to_markdown();
    assert_eq!(markdown, output);
}

#[test]
fn blockquotes() {
    let markdown = "> This is a quote\n> That spans multiple lines";
    let doc = Document::parse(markdown).expect("parse");
    let output = doc.to_markdown();
    assert_eq!(markdown, output);
}

#[test]
fn blockquotes_with_formatting() {
    let markdown = "> This quote has **bold** and *italic* text";
    let doc = Document::parse(markdown).expect("parse");
    let output = doc.to_markdown();
    assert_eq!(markdown, output);
}

#[test]
fn links() {
    let markdown = "Check out [Google](https://google.com) for search.";
    let doc = Document::parse(markdown).expect("parse");
    let output = doc.to_markdown();
    assert_eq!(markdown, output);
}

#[test]
fn tables_basic() {
    let markdown = "|Name|Age|\n|John|25|\n|Jane|30|";
    let doc = Document::parse(markdown).expect("parse");
    let output = doc.to_markdown();
    assert_eq!(markdown, output);
}

#[test]
fn tables_with_headers() {
    let markdown = "|Name|Age|\n|---|---|\n|John|25|\n|Jane|30|";
    let doc = Document::parse(markdown).expect("parse");
    let output = doc.to_markdown();
    assert_eq!(markdown, output);
}

#[test]
fn complex_document() {
    let markdown = "# My Document\n\nThis is a **paragraph** with *formatting*.\n\n## Code Example\n\n```rust\nfn hello() {\n    println!(\"Hello!\");\n}\n```\n\n> This is a quote\n\n- List item 1\n- List item 2\n\n|Col1|Col2|\n|---|---|\n|A|B|";
    let doc = Document::parse(markdown).expect("parse");
    let output = doc.to_markdown();
    assert_eq!(markdown, output);
}

#[test]
fn empty_document() {
    let markdown = "";
    let doc = Document::parse(markdown).expect("parse");
    let output = doc.to_markdown();
    assert_eq!(markdown, output);
}

#[test]
fn whitespace_handling() {
    let markdown = "Paragraph 1\n\nParagraph 2";
    let doc = Document::parse(markdown).expect("parse");
    let output = doc.to_markdown();
    assert_eq!(markdown, output);
}

#[test]
fn nested_bold_italic_case1() {
    let markdown = "**bold *italic* bold**";
    let doc = Document::parse(markdown).expect("parse");
    let output = doc.to_markdown();
    assert_eq!(markdown, output);
}

#[test]
fn nested_italic_bold_case2() {
    let markdown = "*italic **bold** italic*";
    let doc = Document::parse(markdown).expect("parse");
    let output = doc.to_markdown();
    assert_eq!(markdown, output);
}

#[test]
fn triple_star_case1() {
    let markdown = "***bold and italic* just bold**";
    let doc = Document::parse(markdown).expect("parse");
    let output = doc.to_markdown();
    assert_eq!(markdown, output);
}

#[test]
fn triple_star_case2() {
    let markdown = "***bold and italic** just italic*";
    let doc = Document::parse(markdown).expect("parse");
    let output = doc.to_markdown();
    assert_eq!(markdown, output);
}

#[test]
fn nested_formatting_with_code() {
    let markdown = "**bold `code` bold**";
    let doc = Document::parse(markdown).expect("parse");
    let output = doc.to_markdown();
    assert_eq!(markdown, output);
}

#[test]
fn debug_triple_star_decisions() {
    use pinniped_core::inline::{simulate_balance_debug};
    
    // Test case 1: Should be ** + * (bold containing italic)
    let markdown1 = "***bold and italic* just bold**";
    println!("=== Test 1: {} ===", markdown1);
    let bold_first_score = simulate_balance_debug(markdown1, true);
    let italic_first_score = simulate_balance_debug(markdown1, false);
    println!("Bold first score: {}", bold_first_score);
    println!("Italic first score: {}", italic_first_score);
    
    let doc1 = Document::parse(markdown1).expect("parse");
    let output1 = doc1.to_markdown();
    println!("Output: {}", output1);
    assert_eq!(markdown1, output1);
    
    // Test case 2: Should be * + ** (italic containing bold)  
    let markdown2 = "***bold and italic** just italic*";
    println!("\n=== Test 2: {} ===", markdown2);
    let bold_first_score2 = simulate_balance_debug(markdown2, true);
    let italic_first_score2 = simulate_balance_debug(markdown2, false);
    println!("Bold first score: {}", bold_first_score2);
    println!("Italic first score: {}", italic_first_score2);
    
    let doc2 = Document::parse(markdown2).expect("parse");
    let output2 = doc2.to_markdown();
    println!("Output: {}", output2);
    assert_eq!(markdown2, output2);
}
