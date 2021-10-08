use crate::dataframe::DataFrame;
use crate::dataframe::Series;

use super::floats::SeriesF64Reader;
use super::integers::SeriesI32Reader;
use super::strings::SeriesStrReader;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::File;

#[derive(Serialize, Deserialize)]
pub enum SeriesReader {
    Integers(SeriesI32Reader),
    Floats(SeriesF64Reader),
    Strings(SeriesStrReader),
}

#[wasm_bindgen]
pub struct DataFrameReader {
    data: HashMap<String, SeriesReader>,
    index: Vec<String>,
    num_rows: usize,
    num_cols: usize,
}

#[wasm_bindgen]
impl DataFrameReader {
    pub async fn read_csv(data: File) -> Result<DataFrameReader, JsValue> {
        let js_data = wasm_bindgen_futures::JsFuture::from(data.text())
            .await
            .unwrap();

        let data = js_data.as_string().unwrap_throw();

        let mut reader = csv::Reader::from_reader(data.as_bytes());

        let headers: Vec<String> = reader
            .headers()
            .unwrap_throw()
            .clone()
            .into_iter()
            .map(|x| x.to_string())
            .collect();

        let mut data_map: HashMap<String, SeriesReader> = HashMap::new();
        let mut rtc_map: HashMap<usize, Vec<String>> = HashMap::new();

        for row in reader.records() {
            let row_res = row.unwrap_throw();
            for (index, value) in row_res.iter().enumerate() {
                rtc_map
                    .entry(index)
                    .or_insert(Vec::new())
                    .push(value.to_string());
            }

            for key in rtc_map.keys() {
                let col_name = headers[*key].clone();

                let as_int = rtc_map[key][0].parse::<i32>();
                if let Ok(_x) = as_int {
                    let mut int_vec: Vec<Option<i32>> = Vec::new();
                    for int_data in &rtc_map[key] {
                        match int_data.parse::<i32>() {
                            Ok(i) => int_vec.push(Some(i)),
                            Err(_) => int_vec.push(None),
                        }
                    }

                    data_map.insert(
                        headers[*key].clone(),
                        SeriesReader::Integers(SeriesI32Reader::new(col_name, int_vec)),
                    );
                    continue;
                }

                let as_float = rtc_map[key][0].parse::<f32>();
                if let Ok(_x) = as_float {
                    let mut float_vec: Vec<Option<f64>> = Vec::new();
                    for float_data in &rtc_map[key] {
                        match float_data.parse::<f64>() {
                            Ok(i) => float_vec.push(Some(i)),
                            Err(_) => float_vec.push(None),
                        }
                    }

                    data_map.insert(
                        headers[*key].clone(),
                        SeriesReader::Floats(SeriesF64Reader::new(col_name, float_vec)),
                    );
                    continue;
                }

                let _as_str = rtc_map[key][0].parse::<String>();
                if let Ok(_x) = as_float {
                    let mut str_vec: Vec<Option<String>> = Vec::new();
                    for str_data in &rtc_map[key] {
                        match str_data.parse::<String>() {
                            Ok(i) => str_vec.push(Some(i)),
                            Err(_) => str_vec.push(None),
                        }
                    }

                    data_map.insert(
                        headers[*key].clone(),
                        SeriesReader::Strings(SeriesStrReader::new(col_name, str_vec)),
                    );
                    continue;
                }
            }
        }

        let num_rows = rtc_map[&0].len();
        let num_cols = data_map.keys().len();

        Ok(DataFrameReader {
            data: data_map,
            index: headers,
            num_rows,
            num_cols,
        })
    }

    pub fn to_dataframe(&self) -> DataFrame {
        let mut data: HashMap<String, Series> = HashMap::new();
        let mut index: Vec<String> = Vec::new();

        for (_name, series_reader) in &self.data {
            match &series_reader {
                &SeriesReader::Floats(floats) => {
                    index.push(floats.get_name());
                    data.insert(floats.get_name(), Series::Floats(floats.to_series()));
                }
                &SeriesReader::Integers(integers) => {
                    index.push(integers.get_name());
                    data.insert(integers.get_name(), Series::Integers(integers.to_series()));
                }
                &SeriesReader::Strings(strings) => {
                    index.push(strings.get_name());
                    data.insert(strings.get_name(), Series::Strings(strings.to_series()));
                }
            }
        }

        DataFrame::new_rs(index, data, self.num_rows, self.num_cols)
    }
}
