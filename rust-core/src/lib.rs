pub mod block;
pub mod document;
pub mod error;
pub mod ffi;
pub mod inline;
pub mod parser;
pub mod table;

// Re-export main types for easier access
pub use document::Document;
pub use block::Block;
pub use error::Error;
