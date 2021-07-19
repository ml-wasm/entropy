pub mod math;
pub mod series_i32;

use linalg::vectors::integers::IntegersVector;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct SeriesI32 {
    name: String,
    data: IntegersVector,
}
