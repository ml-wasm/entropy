use super::SeriesI32;
use linalg::vectors::integers::IntegersVector;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl SeriesI32 {
    #[wasm_bindgen(js_name = newWithElement)]
    pub fn new_with_element(name: JsValue, len: usize, element: i32) -> SeriesI32 {
        let col_name = serde_wasm_bindgen::from_value(name).unwrap();
        let col_data = IntegersVector::new_with_elem(len, element);

        SeriesI32 {
            name: col_name,
            data: col_data,
        }
    }

    #[wasm_bindgen(js_name = "newWithSimpleFunc")]
    pub fn new_with_simple_func(name: JsValue, len: usize, js_func: js_sys::Function) -> SeriesI32 {
        let col_name = serde_wasm_bindgen::from_value(name).unwrap();
        let col_data = IntegersVector::new_with_simple_func(len, js_func);

        SeriesI32 {
            name: col_name,
            data: col_data,
        }
    }
}
