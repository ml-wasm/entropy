use super::SeriesI32;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl SeriesI32 {
    pub fn sum(&self) -> i32 {
        self.data.sum()
    }

    pub fn product(&self) -> i32 {
        self.data.product()
    }

    pub fn mean(&self) -> f64 {
        let mut sum = 0;
        
        for x in &self.data.data {
            sum += x;
        }

        sum as f64 / self.data.len() as f64
    }

    pub fn median(&self) -> f64 {
        self.data.median()
    }

    pub fn max(&self) -> i32 {
        self.data.max()
    }

    pub fn min(&self) -> i32 {
        self.data.min()
    }

    /// Return varaince of series with given degree of freedom
    pub fn variance(&self, degree_of_freedom: f64) -> f64 {
        self.data.var(degree_of_freedom)
    }

    /// Return standard deviation of series with given degree of freedom
    pub fn std_dev(&self, degree_of_freedom: f64) -> f64 {
        self.data.std(degree_of_freedom)
    }
}
