use wasm_bindgen::prelude::*;

#[wasm_bindgen]
struct SeriesF64Reader
{
    name: String,
    data: Vec<Option<f64>>,
}
