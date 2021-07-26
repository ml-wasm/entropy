use super::DataFrame;
use super::Series;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

macro_rules! operation {
    ($fn_name: ident) => {
        #[wasm_bindgen]
        impl DataFrame {
            /// Returns $fn_name of given column
            pub fn $fn_name(&self, col: JsValue) -> f64 {
                let col_name: String = serde_wasm_bindgen::from_value(col).unwrap();

                if self.data.contains_key(&col_name) {
                    match &self.data[&col_name] {
                        Series::Floats(value) => {
                            return value.$fn_name();
                        }
                        Series::Integers(value) => {
                            return value.$fn_name() as f64;
                        }
                        Series::Strings(_value) => {
                            panic!("min function not supported for strings");
                        }
                    }
                } else {
                    panic!("Column name {} not found", col_name);
                }
            }
        }
    };
}

operation!(min);
operation!(max);
operation!(mean);
operation!(median);

#[wasm_bindgen]
impl DataFrame {
    /// Returns minimun of all columns
    #[wasm_bindgen(js_name = minColumns)]
    pub fn min_colunmns(&self) -> JsValue {
        let mut res: HashMap<String, f64> = HashMap::new();

        for (name, ser) in &self.data {
            match ser {
                Series::Floats(value) => {
                    res.entry(name.clone()).or_insert(value.min());
                }
                Series::Integers(value) => {
                    res.entry(name.clone()).or_insert(value.min() as f64);
                }
                _ => {}
            }
        }

        serde_wasm_bindgen::to_value(&res).unwrap()
    }

    /// Returns maximum elements of all colunms
    #[wasm_bindgen(js_name = maxColumns)]
    pub fn max_columns(&self) -> JsValue {
        let mut res: HashMap<String, f64> = HashMap::new();

        for (name, ser) in &self.data {
            match &ser {
                Series::Floats(value) => {
                    res.entry(name.clone()).or_insert(value.max());
                }
                Series::Integers(value) => {
                    res.entry(name.clone()).or_insert(value.max() as f64);
                }
                _ => {}
            }
        }

        serde_wasm_bindgen::to_value(&res).unwrap()
    }

    /// Returns mean of all columns
    #[wasm_bindgen(js_name = meanColumns)]
    pub fn mean_columns(&self) -> JsValue {
        let mut res: HashMap<String, f64> = HashMap::new();
        for (name, ser) in &self.data {
            match ser {
                Series::Floats(value) => {
                    res.entry(name.clone()).or_insert(value.mean());
                }
                Series::Integers(value) => {
                    res.entry(name.clone()).or_insert(value.mean());
                }
                _ => {}
            }
        }

        serde_wasm_bindgen::to_value(&res).unwrap()
    }

    /// Returns median of all columns
    #[wasm_bindgen(js_name = medianColumns)]
    pub fn median_columns(&self) -> JsValue {
        let mut res: HashMap<String, f64> = HashMap::new();

        for (name, ser) in &self.data {
            match &ser {
                Series::Floats(value) => {
                    res.entry(name.clone()).or_insert(value.median());
                }
                Series::Integers(value) => {
                    res.entry(name.clone()).or_insert(value.median());
                }
                _ => {}
            }
        }

        serde_wasm_bindgen::to_value(&res).unwrap()
    }

    /// Returns variance of given column
    pub fn variance(&self, col: JsValue, degree_of_freedom: f64) -> f64 {
        let col_name: String = serde_wasm_bindgen::from_value(col).unwrap();

        if self.data.contains_key(&col_name) {
            match &self.data[&col_name] {
                Series::Floats(value) => return value.variance(degree_of_freedom),
                Series::Integers(value) => return value.variance(degree_of_freedom),
                Series::Strings(_value) => {
                    panic!("Varinance not supported for Strings");
                }
            }
        }

        panic!("Column name {} not found", col_name)
    }

    /// Returns variance of columns
    #[wasm_bindgen(js_name = varianceColumns)]
    pub fn variance_columns(&self, degree_of_freedom: f64) -> JsValue {
        let mut res: HashMap<String, f64> = HashMap::new();

        for (name, ser) in &self.data {
            match &ser {
                Series::Floats(value) => {
                    res.entry(name.clone())
                        .or_insert(value.variance(degree_of_freedom));
                }
                Series::Integers(value) => {
                    res.entry(name.clone())
                        .or_insert(value.variance(degree_of_freedom));
                }
                _ => {}
            }
        }

        serde_wasm_bindgen::to_value(&res).unwrap()
    }

    /// Returns standard deviation of given column
    #[wasm_bindgen(js_name = standardDeviation)]
    pub fn std_dev(&self, col: JsValue, degree_of_freedom: f64) -> f64 {
        let col_name: String = serde_wasm_bindgen::from_value(col).unwrap();

        if self.data.contains_key(&col_name) {
            match &self.data[&col_name] {
                Series::Floats(value) => return value.std_dev(degree_of_freedom),
                Series::Integers(value) => return value.std_dev(degree_of_freedom),
                Series::Strings(_value) => {
                    panic!("Standard deviation not supported for strings");
                }
            }
        }

        panic!("Column name {} not found", col_name)
    }

    #[wasm_bindgen(js_name = standardDeviationColumns)]
    pub fn std_dev_columns(&self, degree_of_freedom: f64) -> JsValue {
        let mut res: HashMap<String, f64> = HashMap::new();

        for (name, ser) in &self.data {
            match &ser {
                Series::Floats(value) => {
                    res.entry(name.clone())
                        .or_insert(value.std_dev(degree_of_freedom));
                }
                Series::Integers(value) => {
                    res.entry(name.clone())
                        .or_insert(value.std_dev(degree_of_freedom));
                }
                _ => {}
            }
        }

        serde_wasm_bindgen::to_value(&res).unwrap()
    }
}
