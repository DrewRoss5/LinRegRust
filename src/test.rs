use crate::regression::{Regression, DataPoint};
use std::sync::Arc;

// calculates a model's mean squared from the given test data
pub fn test_r2(model: Arc<Regression>, data: Arc<Vec<DataPoint>>) -> f64 {
    let mut err_sum: f64 = 0.0;
    for point in data.iter() {
        let err = point.y - model.predict(point.x);
        err_sum += err * err;
    }
    return err_sum / (data.len() as f64);
}

// calculates a model's mean absolute error from the given test data
pub fn test_mae(model: Arc<Regression>, data: Arc<Vec<DataPoint>>) -> f64 {
    let mut err_sum: f64 = 0.0;
    for point in data.iter() {
        let err = point.y - model.predict(point.x);
        err_sum += err.abs();
    }
    return err_sum / (data.len() as f64);
}