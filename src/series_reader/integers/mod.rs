use wasm_bindgen::prelude::*;

#[wasm_bindgen]
struct SeriesI32Reader
{
    name: String,
    data: Vec<Option<i32>>,
}
