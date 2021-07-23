use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct SeriesStrReader
{
    name: String,
    data: Vec<Option<String>>,
}

impl SeriesStrReader {
    pub fn new(name: String, data: Vec<Option<String>>) -> SeriesStrReader {
        SeriesStrReader {
            name,
            data,
        }
    }
}
