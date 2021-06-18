use linalg::one_dimensional::strings::Strings1d;
pub mod series_str;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct SeriesSTR {
    name: String,
    data: Strings1d,
}
