use super::SeriesF64;
use crate::dataframe::ColumnType;
use linalg::vectors::floats::FloatsVector;
use prettytable::{Cell, Row, Table};
use wasm_bindgen::prelude::*;

impl SeriesF64 {
    pub fn new_rs(name: String, data: Vec<f64>) -> SeriesF64 {
        let col_data = FloatsVector::new(data);

        SeriesF64 {
            name,
            data: col_data,
        }
    }
}

#[wasm_bindgen]
impl SeriesF64 {
    #[wasm_bindgen(constructor)]
    pub fn new(name: JsValue, data: JsValue) -> SeriesF64 {
        let col_name = serde_wasm_bindgen::from_value(name).unwrap();
        let serde_data: Vec<f64> = serde_wasm_bindgen::from_value(data).unwrap();
        let col_data = FloatsVector::new(serde_data);

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

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn dtype(&self) -> ColumnType {
        ColumnType::FLOAT
    }

    pub fn shape(&self) -> JsValue {
        self.data.shape_to_js()
    }

    pub fn data(&self) -> JsValue {
        self.data.data_to_js()
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        self.data.to_string()
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

    pub fn reversed(&self) -> FloatsVector {
        self.data.reversed()
    }

    pub fn append(&mut self, element: f64) {
        self.data.append(element);
    }

    pub fn appended(&mut self, element: f64) -> FloatsVector {
        self.data.appended(element)
    }

    pub fn extend(&mut self, data_arr: JsValue) {
        let data_arr = serde_wasm_bindgen::from_value(data_arr).unwrap();
        let ndarray_data_arr = FloatsVector::new(data_arr);
        self.data.extend(ndarray_data_arr)
    }

    pub fn extended(&mut self, data_arr: JsValue) -> FloatsVector {
        let data_arr = serde_wasm_bindgen::from_value(data_arr).unwrap();
        let ndarray_data_arr = FloatsVector::new(data_arr);
        self.data.extended(ndarray_data_arr)
    }

    pub fn insert(&mut self, index: usize, value: f64) {
        self.data.insert(index, value);
    }

    pub fn inserted(&mut self, index: usize, value: f64) -> FloatsVector {
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

    #[wasm_bindgen(getter,js_name = display)]
    pub fn show(&self) -> String {
        let mut table = Table::new();
        let col_name = self.name.clone();
        table.add_row(row![col_name]);
        for i in 0..self.len() {
            let val = self.data.data[i];
            table.add_row(Row::new(vec![Cell::new(&val.to_string())]));
        }

        table.to_string()
    }
}
