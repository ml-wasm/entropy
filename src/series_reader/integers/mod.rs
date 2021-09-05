use crate::series::integers::SeriesI32;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct SeriesI32Reader {
    name: String,
    data: Vec<Option<i32>>,
}

impl SeriesI32Reader {
    pub fn new(name: String, data: Vec<Option<i32>>) -> SeriesI32Reader {
        SeriesI32Reader { name, data }
    }

    pub fn to_series(&self) -> SeriesI32 {
        let mut data_vec: Vec<i32> = Vec::new();
        data_vec.reserve(self.data.len());

        for value in &self.data {
            match value {
                Some(value) => data_vec.push(*value),
                None => panic!("Null value encountered"),
            }
        }

        SeriesI32::new_rs(self.name.clone(), data_vec)
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
