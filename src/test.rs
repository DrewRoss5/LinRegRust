use crate::regression::{Regression, DataPoint};

// calculates a model's mean squared from the given test data
pub fn test_r2(model: &Regression, data: &Vec<DataPoint>) -> f64 {
    let mut err_sum: f64 = 0.0;
    for point in data {
        let err = point.y - model.predict(point.x);
        err_sum += (err * err);
    }
    return err_sum / (data.len() as f64);;
}

// calculates a model's mean absolute error from the given test data
pub fn test_mae(model: &Regression, data: &Vec<DataPoint>) -> f64 {
    let mut err_sum: f64 = 0.0;
    for point in data {
        let err = point.y - model.predict(point.x);
        err_sum += err.abs();
    }
    return err_sum / (data.len() as f64);
}
