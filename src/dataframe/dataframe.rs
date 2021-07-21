use std::collections::HashMap;

use super::ColumnType;
use super::DataFrame;
use super::Series;
use crate::series::floats::SeriesF64;
use crate::series::integers::SeriesI32;
use crate::series::strings::SeriesSTR;
use ndarray::Data;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl DataFrame {
    #[wasm_bindgen(constructor)]
    pub fn new(vec_series: Vec<JsValue>) -> DataFrame {
        //Get first Series data size

        let mut num_rows = 0;
        let first_series_int: Result<SeriesI32, serde_wasm_bindgen::Error> =
            serde_wasm_bindgen::from_value(vec_series[0].clone());
        if let Ok(series_int) = first_series_int {
            num_rows = series_int.len()
        }

        let first_series_float: Result<SeriesF64, serde_wasm_bindgen::Error> =
            serde_wasm_bindgen::from_value(vec_series[0].clone());
        if let Ok(series_float) = first_series_float {
            num_rows = series_float.len()
        }

        let first_series_str: Result<SeriesSTR, serde_wasm_bindgen::Error> =
            serde_wasm_bindgen::from_value(vec_series[0].clone());
        if let Ok(series_str) = first_series_str {
            num_rows = series_str.len()
        }

        let mut series_data: HashMap<String, Series> = HashMap::new();
        let mut index: Vec<String> = Vec::new();
        let mut num_cols: usize = 0;
        for ser in &vec_series {
            let as_int: Result<SeriesI32, serde_wasm_bindgen::Error> =
                serde_wasm_bindgen::from_value(ser.clone());

            if let Ok(x) = as_int {
                let col_name = x.name();
                if x.len() == num_rows {
                    series_data
                        .entry(col_name.clone())
                        .or_insert(Series::Integers(x));
                    index.push(col_name);
                    num_cols += 1;
                    continue;
                } else {
                    panic!("Series length does not match");
                }
            }

            let as_float: Result<SeriesF64, serde_wasm_bindgen::Error> =
                serde_wasm_bindgen::from_value(ser.clone());

            if let Ok(x) = as_float {
                let col_name = x.name();
                if x.len() == num_rows {
                    series_data
                        .entry(col_name.clone())
                        .or_insert(Series::Floats(x));
                    index.push(col_name);
                    num_cols += 1;
                    continue;
                } else {
                    panic!("Series length does not match");
                }
            }

            let as_str: Result<SeriesSTR, serde_wasm_bindgen::Error> =
                serde_wasm_bindgen::from_value(ser.clone());

            if let Ok(x) = as_str {
                let col_name = x.name();
                if x.len() == num_rows {
                    series_data
                        .entry(col_name.clone())
                        .or_insert(Series::Strings(x));
                    index.push(col_name);
                    num_cols += 1;
                    continue;
                } else {
                    panic!("Series length does not match");
                }
            }
        }

        DataFrame {
            data: series_data,
            index,
            num_rows,
            num_cols,
        }
    }

    #[wasm_bindgen(js_name = columns)]
    pub fn show_columns(&self) -> JsValue {
        let res: Vec<String> = self.index.clone();

        serde_wasm_bindgen::to_value(&res).unwrap()
    }

    #[wasm_bindgen(js_name = dataTypes)]
    pub fn get_datatypes(&self) -> JsValue {
        let mut res: HashMap<String, ColumnType> = HashMap::new();

        for (name, ser) in &self.data {
            match ser {
                Series::Floats(_value) => {
                    res.entry(name.clone()).or_insert(ColumnType::FLOAT);
                }
                Series::Integers(_value) => {
                    res.entry(name.clone()).or_insert(ColumnType::INTEGER);
                }
                Series::Strings(_value) => {
                    res.entry(name.clone()).or_insert(ColumnType::STR);
                }
            }
        }

        serde_wasm_bindgen::to_value(&res).unwrap()
    }

    #[wasm_bindgen(js_name = append)]
    pub fn add_column(&mut self, datatype: ColumnType, series: JsValue) {
        match datatype {
            ColumnType::FLOAT => {
                let ser: SeriesF64 = serde_wasm_bindgen::from_value(series).unwrap();
                self.data.entry(ser.name()).or_insert(Series::Floats(ser));
            }
            ColumnType::INTEGER => {
                let ser: SeriesI32 = serde_wasm_bindgen::from_value(series).unwrap();
                self.data.entry(ser.name()).or_insert(Series::Integers(ser));
            }
            ColumnType::STR => {
                let ser: SeriesSTR = serde_wasm_bindgen::from_value(series).unwrap();
                self.data.entry(ser.name()).or_insert(Series::Strings(ser));
            }
        }
    }

    #[wasm_bindgen(js_name = rowsCount)]
    pub fn num_rows(&self) -> usize {
        self.num_rows
    }

    #[wasm_bindgen(js_name = columnsCount)]
    pub fn num_cols(&self) -> usize {
        self.num_cols
    }

    #[wasm_bindgen(js_name = size)]
    pub fn dataframe_size(&self) -> usize {
        self.data.iter().count()
    }

    pub fn loc(&self, column_name: String) -> String {
        let res;
        let map = &self.data;
        let ser = &map[&column_name];
        match ser {
            Series::Integers(x) => {
                res = x.show().to_string();
            }
            Series::Floats(x) => {
                res = x.show().to_string();
            }
            Series::Strings(x) => {
                res = x.show().to_string();
            }
        };

        res
    }

    pub fn ilocr(&self, row: usize) -> js_sys::Array {
        let array = js_sys::Array::new();
        let map = &self.data;
        self.index.iter().for_each(|f| {
            let ser = &map[f];
            match ser {
                Series::Integers(x) => {
                    let val = serde_wasm_bindgen::to_value(&x.get(row)).unwrap();
                    array.push(&val);
                }
                Series::Floats(x) => {
                    let val = serde_wasm_bindgen::to_value(&x.get(row)).unwrap();
                    array.push(&val);
                }
                Series::Strings(x) => {
                    let val = serde_wasm_bindgen::to_value(&x.get(row)).unwrap();
                    array.push(&val);
                }
            };
        });

        array
    }

    pub fn ilocc(&self, col: usize) -> JsValue {
        let val: JsValue;
        let map = &self.data;
        let col_idx_name = &self.index[col];
        let ser = &map[col_idx_name];
        match ser {
            Series::Integers(x) => {
                val = x.data();
            }
            Series::Floats(x) => {
                val = x.data();
            }
            Series::Strings(x) => {
                val = x.data();
            }
        };
        val
    }

    #[wasm_bindgen(getter,js_name = display)]
    pub fn show(&self) -> String {
        let map = &self.data;
        let mut res: String = String::from("");
        self.index.iter().for_each(|f| {
            let ser = &map[f];
            match &ser {
                Series::Integers(value) => {
                    res.push_str(&value.show());
                }
                Series::Floats(value) => {
                    res.push_str(&value.show());
                }
                Series::Strings(value) => {
                    res.push_str(&value.show());
                }
            }
        });
        res
    }

    #[wasm_bindgen(getter,js_name = displayTable)]
    pub fn show_table(&self) -> js_sys::Array {
        let n = self.num_rows();
        let array_col = js_sys::Array::new();
        let map = &self.data;
        for i in 0..n {
            let array_row = js_sys::Array::new();
            self.index.iter().for_each(|f| {
                let ser = &map[f];
                match ser {
                    Series::Integers(x) => {
                        let val = serde_wasm_bindgen::to_value(&x.get(i)).unwrap();
                        array_row.push(&val);
                    }
                    Series::Floats(x) => {
                        let val = serde_wasm_bindgen::to_value(&x.get(i)).unwrap();
                        array_row.push(&val);
                    }
                    Series::Strings(x) => {
                        let val = serde_wasm_bindgen::to_value(&x.get(i)).unwrap();
                        array_row.push(&val);
                    }
                };
            });
            array_col.push(&array_row);
        }

        array_col
    }
}

#[wasm_bindgen(js_name = readcsv)]
pub async fn read_csv(data: web_sys::File) -> Result<DataFrame, JsValue> {
    let jsdata = wasm_bindgen_futures::JsFuture::from(data.text())
        .await
        .unwrap_throw();

    let data = jsdata.as_string().unwrap_throw();

    let mut reader = csv::Reader::from_reader(data.as_bytes());

    let headers: Vec<String> = reader
        .headers()
        .unwrap_throw()
        .clone()
        .into_iter()
        .map(|x| x.to_string())
        .collect();

    let mut data_map: HashMap<String, Series> = HashMap::new();
    let mut rtc_map: HashMap<usize, Vec<String>> = HashMap::new();
    reader.records().for_each(|row| {
        let row_res = row.unwrap_throw();
        row_res.iter().enumerate().for_each(|(index, value)| {
            rtc_map
                .entry(index)
                .or_insert(Vec::new())
                .push(value.to_string());
        });

        rtc_map.keys().for_each(|key| {
            let col_name = headers[*key].clone();
            data_map.insert(
                headers[*key].clone(),
                Series::Strings(SeriesSTR::new(
                    serde_wasm_bindgen::to_value(&col_name).unwrap(),
                    serde_wasm_bindgen::to_value(&rtc_map[key]).unwrap(),
                )),
            );
        })
    });

    // Ok(serde_wasm_bindgen::to_value(&data_map).unwrap())
    Ok(DataFrame {
        data: data_map,
        index: headers,
        num_rows: 500,
        num_cols: 4,
    })
}
