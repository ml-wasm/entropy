use linalg::one_dimensional::integers::Integers1d;
pub mod math;
pub mod series_i32;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct SeriesI32 {
    name: String,
    data: Integers1d,
}
