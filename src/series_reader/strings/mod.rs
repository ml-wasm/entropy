use linalg::vectors::strings::StringsVector;
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

use crate::series::strings::SeriesSTR;

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

    pub fn to_series(&self) -> SeriesSTR {
        let mut data_vec: Vec<String> = Vec::new();
        data_vec.reserve(self.data.len());

        for value in &self.data {
            match value {
                Some(value) => data_vec.push(value.to_string()),
                None => panic!("Null value encountered")
            }
        }

        SeriesSTR::new_rs(self.name.clone(), data_vec)
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
