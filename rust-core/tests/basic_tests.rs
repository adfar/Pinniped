use pinniped_core::document::Document;

#[test]
fn test_basic_parsing() {
    let markdown = "# Hello World";
    let result = Document::parse(markdown);
    assert!(result.is_ok());
    
    let document = result.unwrap();
    assert_eq!(document.blocks.len(), 1);
}

#[test] 
fn test_serialization() {
    let markdown = "# Test\n\nParagraph";
    let document = Document::parse(markdown).unwrap();
    
    // Test that we can serialize to JSON
    let json = serde_json::to_string(&document).unwrap();
    assert!(!json.is_empty());
    
    // Test that we can deserialize from JSON
    let deserialized: Document = serde_json::from_str(&json).unwrap();
    assert_eq!(document, deserialized);
}