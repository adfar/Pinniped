use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use crate::document::Document;
use crate::block::Block;
use crate::table::Table;

/// Parse markdown text and return document as JSON
#[no_mangle]
pub extern "C" fn pinniped_parse_markdown(input: *const c_char) -> *mut c_char {
    if input.is_null() {
        let error = serde_json::json!({ "error": "Input cannot be null" });
        return CString::new(error.to_string()).unwrap().into_raw();
    }
    
    let input = unsafe { 
        match CStr::from_ptr(input).to_str() {
            Ok(s) => s,
            Err(_) => {
                let error = serde_json::json!({ "error": "Invalid UTF-8 in input" });
                return CString::new(error.to_string()).unwrap().into_raw();
            }
        }
    };
    
    match Document::parse(input) {
        Ok(document) => {
            match serde_json::to_string(&document) {
                Ok(json) => {
                    match CString::new(json) {
                        Ok(c_str) => c_str.into_raw(),
                        Err(_) => {
                            let error = serde_json::json!({ "error": "Failed to create C string" });
                            CString::new(error.to_string()).unwrap().into_raw()
                        }
                    }
                }
                Err(e) => {
                    let error = serde_json::json!({ "error": format!("JSON serialization failed: {}", e) });
                    CString::new(error.to_string()).unwrap().into_raw()
                }
            }
        }
        Err(e) => {
            let error = serde_json::json!({ "error": e.to_string() });
            CString::new(error.to_string()).unwrap().into_raw()
        }
    }
}

/// Convert document JSON back to markdown
#[no_mangle]
pub extern "C" fn pinniped_to_markdown(document_json: *const c_char) -> *mut c_char {
    if document_json.is_null() {
        let error = serde_json::json!({ "error": "Document JSON cannot be null" });
        return CString::new(error.to_string()).unwrap().into_raw();
    }
    
    let json_str = unsafe { 
        match CStr::from_ptr(document_json).to_str() {
            Ok(s) => s,
            Err(_) => {
                let error = serde_json::json!({ "error": "Invalid UTF-8 in document JSON" });
                return CString::new(error.to_string()).unwrap().into_raw();
            }
        }
    };
    
    match serde_json::from_str::<Document>(json_str) {
        Ok(document) => {
            let markdown = document.to_markdown();
            match CString::new(markdown) {
                Ok(c_str) => c_str.into_raw(),
                Err(_) => {
                    let error = serde_json::json!({ "error": "Failed to create C string for markdown" });
                    CString::new(error.to_string()).unwrap().into_raw()
                }
            }
        }
        Err(e) => {
            let error = serde_json::json!({ "error": format!("JSON deserialization failed: {}", e) });
            CString::new(error.to_string()).unwrap().into_raw()
        }
    }
}

/// Navigate within a table at the specified block index
#[no_mangle]
pub extern "C" fn pinniped_table_navigate(
    document_json: *const c_char,
    block_index: i32,
    current_row: i32,
    current_col: i32,
    direction: i32  // 0=up, 1=down, 2=left, 3=right
) -> *mut c_char {
    if document_json.is_null() {
        let error = serde_json::json!({ "error": "Document JSON cannot be null" });
        return CString::new(error.to_string()).unwrap().into_raw();
    }
    
    let json_str = unsafe { 
        match CStr::from_ptr(document_json).to_str() {
            Ok(s) => s,
            Err(_) => {
                let error = serde_json::json!({ "error": "Invalid UTF-8 in document JSON" });
                return CString::new(error.to_string()).unwrap().into_raw();
            }
        }
    };
    
    let document = match serde_json::from_str::<Document>(json_str) {
        Ok(doc) => doc,
        Err(e) => {
            let error = serde_json::json!({ "error": format!("JSON deserialization failed: {}", e) });
            return CString::new(error.to_string()).unwrap().into_raw();
        }
    };
    
    if block_index < 0 || block_index as usize >= document.blocks.len() {
        let error = serde_json::json!({ "error": "Block index out of range" });
        return CString::new(error.to_string()).unwrap().into_raw();
    }
    
    let block = &document.blocks[block_index as usize];
    let table = match block {
        Block::Table(t) => t,
        _ => {
            let error = serde_json::json!({ "error": "Block is not a table" });
            return CString::new(error.to_string()).unwrap().into_raw();
        }
    };
    
    let new_position = calculate_new_position(table, current_row, current_col, direction);
    let result = serde_json::json!(new_position);
    
    match CString::new(result.to_string()) {
        Ok(c_str) => c_str.into_raw(),
        Err(_) => {
            let error = serde_json::json!({ "error": "Failed to create C string for result" });
            CString::new(error.to_string()).unwrap().into_raw()
        }
    }
}

/// Get cell content at specified position
#[no_mangle]
pub extern "C" fn pinniped_table_get_cell(
    document_json: *const c_char,
    block_index: i32,
    row: i32,
    col: i32
) -> *mut c_char {
    if document_json.is_null() {
        let error = serde_json::json!({ "error": "Document JSON cannot be null" });
        return CString::new(error.to_string()).unwrap().into_raw();
    }
    
    let json_str = unsafe { 
        match CStr::from_ptr(document_json).to_str() {
            Ok(s) => s,
            Err(_) => {
                let error = serde_json::json!({ "error": "Invalid UTF-8 in document JSON" });
                return CString::new(error.to_string()).unwrap().into_raw();
            }
        }
    };
    
    let document = match serde_json::from_str::<Document>(json_str) {
        Ok(doc) => doc,
        Err(e) => {
            let error = serde_json::json!({ "error": format!("JSON deserialization failed: {}", e) });
            return CString::new(error.to_string()).unwrap().into_raw();
        }
    };
    
    if block_index < 0 || block_index as usize >= document.blocks.len() {
        let error = serde_json::json!({ "error": "Block index out of range" });
        return CString::new(error.to_string()).unwrap().into_raw();
    }
    
    let block = &document.blocks[block_index as usize];
    let table = match block {
        Block::Table(t) => t,
        _ => {
            let error = serde_json::json!({ "error": "Block is not a table" });
            return CString::new(error.to_string()).unwrap().into_raw();
        }
    };
    
    let actual_row = if table.has_header && row == 0 {
        0  // Header row
    } else if table.has_header {
        (row + 1) as usize  // Skip separator row
    } else {
        row as usize
    };
    
    if actual_row >= table.rows.len() || col < 0 || col as usize >= table.rows[actual_row].len() {
        let error = serde_json::json!({ "error": "Cell position out of range" });
        return CString::new(error.to_string()).unwrap().into_raw();
    }
    
    let cell_content = &table.rows[actual_row][col as usize];
    let result = serde_json::json!({ "content": cell_content });
    
    match CString::new(result.to_string()) {
        Ok(c_str) => c_str.into_raw(),
        Err(_) => {
            let error = serde_json::json!({ "error": "Failed to create C string for cell content" });
            CString::new(error.to_string()).unwrap().into_raw()
        }
    }
}

/// Free a string returned by Rust
#[no_mangle]
pub extern "C" fn pinniped_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}

#[derive(serde::Serialize)]
struct CellPosition {
    row: i32,
    col: i32,
    valid: bool,
}

fn calculate_new_position(table: &Table, current_row: i32, current_col: i32, direction: i32) -> CellPosition {
    let max_row = if table.has_header {
        (table.rows.len() - 2) as i32  // Exclude header and separator rows
    } else {
        (table.rows.len() - 1) as i32
    };
    
    let max_col = if !table.rows.is_empty() {
        (table.rows[0].len() - 1) as i32
    } else {
        0
    };
    
    let (new_row, new_col) = match direction {
        0 => (current_row - 1, current_col), // Up
        1 => (current_row + 1, current_col), // Down
        2 => (current_row, current_col - 1), // Left
        3 => (current_row, current_col + 1), // Right
        _ => (current_row, current_col),     // Invalid direction
    };
    
    let valid = new_row >= 0 && new_row <= max_row && new_col >= 0 && new_col <= max_col;
    
    if valid {
        CellPosition {
            row: new_row,
            col: new_col,
            valid: true,
        }
    } else {
        CellPosition {
            row: current_row,
            col: current_col,
            valid: false,
        }
    }
}