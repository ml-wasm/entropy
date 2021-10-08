use super::SeriesF64;
use crate::dataframe::ColumnType;
use linalg::vectors::floats::FloatsVector;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl SeriesF64 {
    #[wasm_bindgen(js_name = newWithElement)]
    pub fn new_with_element(name: JsValue, len: usize, element: f64) -> SeriesF64 {
        let col_name = serde_wasm_bindgen::from_value(name).unwrap();
        let col_data = FloatsVector::new_with_elem(len, element);

        SeriesF64 {
            name: col_name,
            data: col_data,
        }
    }

    #[wasm_bindgen(js_name = "newWithSimpleFunc")]
    pub fn new_with_simple_func(name: JsValue, len: usize, js_func: js_sys::Function) -> SeriesF64 {
        let col_name = serde_wasm_bindgen::from_value(name).unwrap();
        let col_data = FloatsVector::new_with_simple_func(len, js_func);

        SeriesF64 {
            name: col_name,
            data: col_data,
        }
    }
}
