pub mod series_str;

use linalg::vectors::strings::StringsVector;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct SeriesSTR {
    name: String,
    data: StringsVector,
}
