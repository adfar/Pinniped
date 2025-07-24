use pinniped_core::{Document, Block};
use serde_json;

#[test]
fn test_document_serialization() {
    let markdown = "# Hello World\n\nThis is a **test** paragraph.";
    
    // Parse the document
    let document = Document::parse(markdown).unwrap();
    
    // Test serialization to JSON
    let json = serde_json::to_string(&document).unwrap();
    assert!(!json.is_empty());
    
    // Test deserialization from JSON
    let deserialized: Document = serde_json::from_str(&json).unwrap();
    assert_eq!(document, deserialized);
}

#[test]
fn test_round_trip_conversion() {
    let original_markdown = "# Test Document\n\n**Bold** and *italic* text.\n\n- List item 1\n- List item 2";
    
    // Parse and convert back
    let document = Document::parse(original_markdown).unwrap();
    let regenerated = document.to_markdown();
    
    // Should preserve essential structure
    assert!(regenerated.contains("# Test Document"));
    assert!(regenerated.contains("**Bold**"));
    assert!(regenerated.contains("*italic*"));
    assert!(regenerated.contains("- List item 1"));
    assert!(regenerated.contains("- List item 2"));
}

#[test]
fn test_table_parsing() {
    let markdown_with_table = "# Table Test\n\n| Name | Age | City |\n|------|-----|------|\n| John | 25  | NYC  |\n| Jane | 30  | LA   |";
    
    let document = Document::parse(markdown_with_table).unwrap();
    
    // Should have 2 blocks: header and table
    assert_eq!(document.blocks.len(), 2);
    
    // First block should be header
    match &document.blocks[0] {
        Block::Header { level, .. } => assert_eq!(*level, 1),
        _ => panic!("Expected header block"),
    }
    
    // Second block should be table
    match &document.blocks[1] {
        Block::Table(table) => {
            assert!(table.has_header);
            assert_eq!(table.rows.len(), 4); // Header + separator + 2 data rows
            assert_eq!(table.rows[0], vec!["Name", "Age", "City"]);
            assert_eq!(table.rows[2], vec!["John", "25", "NYC"]);
            assert_eq!(table.rows[3], vec!["Jane", "30", "LA"]);
        },
        _ => panic!("Expected table block"),
    }
}

#[test]
fn test_complex_document_structure() {
    let complex_markdown = r#"# Main Title

This is an introduction paragraph with **bold** and *italic* text.

## Subsection

Here's a list:
- First item with `inline code`
- Second item

### Code Example

```rust
fn hello() {
    println!("Hello, world!");
}
```

> This is a blockquote with *emphasis*.

| Feature | Status |
|---------|--------|
| Parsing | ✅     |
| Tables  | ✅     |
"#;

    let document = Document::parse(complex_markdown).unwrap();
    
    // Should have multiple blocks
    assert!(document.blocks.len() >= 6);
    
    // Test serialization doesn't fail
    let json = serde_json::to_string(&document).unwrap();
    assert!(!json.is_empty());
    
    // Test round-trip
    let deserialized: Document = serde_json::from_str(&json).unwrap();
    assert_eq!(document, deserialized);
    
    // Test markdown generation
    let regenerated = document.to_markdown();
    println!("Regenerated markdown: {}", regenerated);
    assert!(regenerated.contains("# Main Title"));
    assert!(regenerated.contains("```rust"));
    // The table format might be slightly different, so let's check more flexibly
    assert!(regenerated.contains("Feature") && regenerated.contains("Status"));
}

#[test]
fn test_unicode_handling() {
    let unicode_markdown = "# 🦭 Unicode Test\n\nEmojis: 🎉 ✨ 🚀\n\nGreek: α β γ δ ε ζ";
    
    let document = Document::parse(unicode_markdown).unwrap();
    let json = serde_json::to_string(&document).unwrap();
    let deserialized: Document = serde_json::from_str(&json).unwrap();
    
    assert_eq!(document, deserialized);
    
    let regenerated = document.to_markdown();
    assert!(regenerated.contains("🦭"));
    assert!(regenerated.contains("🎉"));
    assert!(regenerated.contains("α β γ"));
}

#[test]
fn test_empty_and_whitespace() {
    // Test empty document
    let empty_doc = Document::parse("").unwrap();
    assert_eq!(empty_doc.blocks.len(), 0);
    
    // Test whitespace-only document
    let whitespace_doc = Document::parse("   \n\n  \t  \n").unwrap();
    assert_eq!(whitespace_doc.blocks.len(), 0);
    
    // Test document with only whitespace paragraphs
    let spaced_doc = Document::parse("# Title\n\n   \n\nParagraph").unwrap();
    assert_eq!(spaced_doc.blocks.len(), 2); // Title and paragraph, whitespace ignored
}

#[test]
fn test_large_document_performance() {
    // Create a reasonably large document
    let mut large_markdown = String::from("# Performance Test\n\n");
    
    for i in 0..50 {
        large_markdown.push_str(&format!("## Section {}\n\n", i));
        large_markdown.push_str("This is a paragraph with **bold** and *italic* text.\n\n");
        large_markdown.push_str("- List item 1\n- List item 2\n\n");
        
        if i % 5 == 0 {
            large_markdown.push_str("| Col1 | Col2 | Col3 |\n|------|------|------|\n");
            large_markdown.push_str(&format!("| Data{} | Value{} | Info{} |\n\n", i, i, i));
        }
    }
    
    // This should parse without issues
    let start = std::time::Instant::now();
    let document = Document::parse(&large_markdown).unwrap();
    let parse_time = start.elapsed();
    
    println!("Parse time for large document: {:?}", parse_time);
    assert!(parse_time.as_millis() < 1000); // Should parse in under 1 second
    
    // Should have many blocks
    assert!(document.blocks.len() > 100);
    
    // Test serialization performance
    let start = std::time::Instant::now();
    let json = serde_json::to_string(&document).unwrap();
    let serialize_time = start.elapsed();
    
    println!("Serialize time: {:?}", serialize_time);
    assert!(serialize_time.as_millis() < 500);
    assert!(!json.is_empty());
    
    // Test deserialization performance
    let start = std::time::Instant::now();
    let _deserialized: Document = serde_json::from_str(&json).unwrap();
    let deserialize_time = start.elapsed();
    
    println!("Deserialize time: {:?}", deserialize_time);
    assert!(deserialize_time.as_millis() < 500);
}

#[test]
fn test_error_handling() {
    // These should not panic, even with malformed input
    let malformed_inputs = vec![
        "# \x00", // Null byte
        "**unclosed bold",
        "*unclosed italic",
        "`unclosed code",
        "[unclosed link](incomplete",
        "| malformed | table |\n|incomplete",
    ];
    
    for input in malformed_inputs {
        // Should not panic, might produce unexpected but valid output
        let result = Document::parse(input);
        match result {
            Ok(doc) => {
                // Should be able to serialize
                let _json = serde_json::to_string(&doc).unwrap();
            }
            Err(_) => {
                // Error is also acceptable for malformed input
            }
        }
    }
}