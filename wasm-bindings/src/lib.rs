use wasm_bindgen::prelude::*;
use pinniped_core::document::Document;

#[wasm_bindgen]
pub fn parse(markdown: &str) -> Result<JsValue, JsValue> {
    let doc = Document::parse(markdown).map_err(|e| JsValue::from_str(&e.to_string()))?;
    JsValue::from_serde(&doc.blocks).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn to_markdown(blocks: &JsValue) -> Result<String, JsValue> {
    let blocks: Vec<pinniped_core::block::Block> = blocks.into_serde().map_err(|e| JsValue::from_str(&e.to_string()))?;
    let doc = Document { blocks };
    Ok(doc.to_markdown())
}
