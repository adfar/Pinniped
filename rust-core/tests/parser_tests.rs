use pinniped_core::document::Document;

#[test]
fn round_trip_paragraph_and_table() {
    let markdown = "Hello world\n\n|a|b|\n|c|d|";
    let doc = Document::parse(markdown).expect("parse");
    let output = doc.to_markdown();
    assert_eq!(markdown, output);
}
