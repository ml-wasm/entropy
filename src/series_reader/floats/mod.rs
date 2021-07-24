use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct SeriesF64Reader
{
    name: String,
    data: Vec<Option<f64>>,
}

impl SeriesF64Reader {
    pub fn new(name: String, data: Vec<Option<f64>>) -> SeriesF64Reader {
        SeriesF64Reader {
            name,
            data
        }
    }
}
