use crate::series::floats::SeriesF64;
use linalg::vectors::floats::FloatsVector;
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

    pub fn  to_series(&self) -> SeriesF64 {
        let mut data_vec: Vec<f64> = Vec::new();
        data_vec.reserve(self.data.len());

        for value in &self.data {
            match value {
                Some(value) => data_vec.push(*value),
                None => panic!("Null value encountered")
            }
        }

        SeriesF64::new_rs(self.name.clone(), data_vec)
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
