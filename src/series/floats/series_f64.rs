use super::SeriesF64;
use crate::dataframe::ColumnType;
use linalg::one_dimensional::floats::Floats1d;
// use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl SeriesF64 {
    #[wasm_bindgen(constructor)]
    pub fn new(name: JsValue, data: JsValue) -> SeriesF64 {
        let col_name = serde_wasm_bindgen::from_value(name).unwrap();
        let serde_data: Vec<f64> = serde_wasm_bindgen::from_value(data).unwrap();
        let col_data = Floats1d::new(serde_data);

        SeriesF64 {
            name: col_name,
            data: col_data,
        }
    }

    #[wasm_bindgen(js_name = toJson)]
    pub fn to_json(&self) -> JsValue {
        let js_series = self;

        serde_wasm_bindgen::to_value(&js_series).unwrap()
    }

    pub fn data(&self) -> JsValue {
        self.data.data_to_js()
    }

    pub fn get(&self, index: usize) -> f64 {
        self.data.get(index)
    }

    pub fn set(&mut self, index: usize, value: f64) {
        self.data.set(index, value);
    }

    pub fn swap(&mut self, index1: usize, index2: usize) {
        self.data.swap(index1, index2);
    }

    pub fn reverse(&mut self) {
        self.data.reverse();
    }

    pub fn reversed(&self) -> Floats1d {
        self.data.reversed()
    }

    pub fn append(&mut self, element: f64) {
        self.data.append(element);
    }

    pub fn appended(&mut self, element: f64) -> Floats1d {
        self.data.appended(element)
    }

    pub fn extend(&mut self, data_arr: JsValue) {
        let data_arr = serde_wasm_bindgen::from_value(data_arr).unwrap();
        let ndarray_data_arr = Floats1d::new(data_arr);
        self.data.extend(ndarray_data_arr)
    }

    pub fn extended(&mut self, data_arr: JsValue) -> Floats1d {
        let data_arr = serde_wasm_bindgen::from_value(data_arr).unwrap();
        let ndarray_data_arr = Floats1d::new(data_arr);
        self.data.extended(ndarray_data_arr)
    }

    pub fn insert(&mut self, index: usize, value: f64) {
        self.data.insert(index, value);
    }

    pub fn inserted(&mut self, index: usize, value: f64) -> Floats1d {
        self.data.inserted(index, value)
    }

    pub fn splice(&mut self, index: usize) -> f64 {
        self.data.splice(index)
    }

    pub fn spliced(&mut self, index: usize) -> js_sys::Array {
        self.data.spliced(index)
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(js_name = updateName)]
    pub fn update_name(&mut self, column_name: JsValue) -> String {
        let column_name = serde_wasm_bindgen::from_value(column_name).unwrap();
        self.name = column_name;

        self.name.clone()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn dtype(&self) -> ColumnType {
        ColumnType::FLOAT
    }

    #[wasm_bindgen(getter,js_name = display)]
    pub fn show(&self) -> String {
        let col_name = self.name.clone();
        let col_name_size = self.name.chars().count();
        let margin = "#".repeat(8 + col_name_size);
        let space = " ".repeat((8 + col_name_size) - col_name_size - 3);

        let mut c = 0;
        let data: String = self
            .data
            .data
            .iter()
            .map(|&x| {
                c += 1;

                if c == self.len() {
                    let x_size = x.to_string().chars().count();
                    let num_space = " ".repeat(8 + col_name_size - x_size - 3);
                    x.to_string() + &num_space + &"#".to_string()
                } else {
                    let x_size = x.to_string().chars().count();
                    let num_space = " ".repeat(8 + col_name_size - x_size - 3);
                    x.to_string() + &num_space + &"#".to_string() + &"\n# ".to_string()
                }
            })
            .collect();
        format!(
            "{}\n# {}{}#\n{}\n# {}\n{}\n",
            margin, col_name, space, margin, data, margin
        )
    }

    // #[wasm_bindgen(getter,js_name = display)]
    // pub fn show(&self) -> JsValue {
    //     #[derive(Serialize, Deserialize)]
    //     struct Display {
    //         name: String,
    //         data: Vec<f64>,
    //     }

    //     let display_series = Display {
    //         name: self.name.clone(),
    //         data: self.data.data.to_vec(),
    //     };
    //     serde_wasm_bindgen::to_value(&display_series).unwrap()
    // }
}
