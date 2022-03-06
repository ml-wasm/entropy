use super::SeriesF64;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl SeriesF64 {
    pub fn min(&self) -> f64 {
        self.data.min()
    }

    pub fn max(&self) -> f64 {
        self.data.max()
    }

    pub fn sum(&self) -> f64 {
        self.data.sum()
    }

    pub fn product(&self) -> f64 {
        self.data.product()
    }

    pub fn mean(&self) -> f64 {
        self.data.mean()
    }

    pub fn median(&self) -> f64 {
        self.data.median()
    }

    /// Return variance of series with given degree of freedom
    pub fn variance(&self, degree_of_freedom: f64) -> f64 {
        self.data.var(degree_of_freedom)
    }

    /// Return standard deviation of series woth given degree of freedom
    pub fn std_dev(&self, degree_of_freedom: f64) -> f64 {
        self.data.std(degree_of_freedom)
    }
}
