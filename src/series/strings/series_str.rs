use super::SeriesSTR;
use crate::dataframe::ColumnType;
use linalg::vectors::strings::StringsVector;
use prettytable::{Cell, Row, Table};
use wasm_bindgen::prelude::*;

impl SeriesSTR {
    pub fn new_rs(name: String, data: Vec<String>) -> SeriesSTR {
        let col_data = StringsVector::new(data);

        SeriesSTR {
            name,
            data: col_data,
        }
    }
}

#[wasm_bindgen]
impl SeriesSTR {
    #[wasm_bindgen(constructor)]
    pub fn new(name: JsValue, data: JsValue) -> SeriesSTR {
        let col_name = serde_wasm_bindgen::from_value(name).unwrap();
        let serde_data: Vec<String> = serde_wasm_bindgen::from_value(data).unwrap();
        let col_data = StringsVector::new(serde_data);

        SeriesSTR {
            name: col_name,
            data: col_data,
        }
    }

    #[wasm_bindgen(js_name = toJson)]
    pub fn to_json(&self) -> JsValue {
        let js_series = self;

        serde_wasm_bindgen::to_value(&js_series).unwrap()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn shape(&self) -> JsValue {
        self.data.shape_to_js()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn dtype(&self) -> ColumnType {
        ColumnType::STR
    }

    pub fn data(&self) -> JsValue {
        self.data.data_to_js()
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        self.data.to_string()
    }

    pub fn get(&self, index: usize) -> String {
        self.data.get(index)
    }

    pub fn set(&mut self, index: usize, value: String) {
        self.data.set(index, value);
    }

    pub fn swap(&mut self, index1: usize, index2: usize) {
        self.data.swap(index1, index2);
    }

    pub fn reverse(&mut self) {
        self.data.reverse();
    }

    pub fn reversed(&self) -> StringsVector {
        self.data.reversed()
    }

    pub fn append(&mut self, element: String) {
        self.data.append(element);
    }

    pub fn appended(&mut self, element: String) -> StringsVector {
        self.data.appended(element)
    }

    pub fn insert(&mut self, index: usize, value: String) {
        self.data.insert(index, value);
    }

    pub fn inserted(&mut self, index: usize, value: String) -> StringsVector {
        self.data.inserted(index, value)
    }

    pub fn splice(&mut self, index: usize) -> String {
        self.data.splice(index)
    }

    pub fn spliced(&mut self, index: usize) -> js_sys::Array {
        self.data.spliced(index)
    }

    pub fn extend(&mut self, data_arr: JsValue) {
        let data_arr = serde_wasm_bindgen::from_value(data_arr).unwrap();
        let ndarray_data_arr = StringsVector::new(data_arr);
        self.data.extend(ndarray_data_arr)
    }

    pub fn extended(&mut self, data_arr: JsValue) -> StringsVector {
        let data_arr = serde_wasm_bindgen::from_value(data_arr).unwrap();
        let ndarray_data_arr = StringsVector::new(data_arr);
        self.data.extended(ndarray_data_arr)
    }

    #[wasm_bindgen(getter,js_name = display)]
    pub fn show(&self) -> String {
        let mut table = Table::new();
        let col_name = self.name.clone();
        table.add_row(row![col_name]);
        for i in 0..self.len() {
            let val = &self.data.data[i];
            table.add_row(Row::new(vec![Cell::new(&val.to_string())]));
        }

        table.to_string()
    }
}
