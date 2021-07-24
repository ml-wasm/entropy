use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct SeriesI32Reader
{
    name: String,
    data: Vec<Option<i32>>,
}

impl SeriesI32Reader {
    pub fn new(name: String, data: Vec<Option<i32>>) -> SeriesI32Reader {
        SeriesI32Reader {
            name,
            data
        }
    }
}
