use wasm_bindgen::prelude::*;

#[wasm_bindgen]
struct SeriesStrReader
{
    name: String,
    data: Vec<Option<String>>,
}
