use crate::series::floats::SeriesF64;
use crate::series::integers::SeriesI32;
use crate::series::strings::SeriesSTR;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

mod dataframe;
mod maths;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub enum ColumnType {
    INTEGER,
    FLOAT,
    STR,
}

#[derive(Serialize, Deserialize)]
pub enum Series {
    Integers(SeriesI32),
    Floats(SeriesF64),
    Strings(SeriesSTR),
}

#[wasm_bindgen]
pub struct DataFrame {
    data: HashMap<String, Series>,
    index: Vec<String>,
    num_rows: usize,
    num_cols: usize
}

