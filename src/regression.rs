use std::{fs::{File, OpenOptions}, io::{Error, ErrorKind}};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct DataPoint {
    pub x: f64,
    pub y: f64
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Regression {
    slope: f64,
    intercept: f64,
    avg: DataPoint,
}
impl Regression {
    // returns a new model all parameters set to 0
    pub fn new() -> Self {
        Self {
            slope: 0.0,
            intercept: 0.0,
            avg: DataPoint { x: 0.0, y: 0.0 },
        }
    }

    // imports a model from a .json file 
    pub fn import_model(file_path: &str) -> Result<Self, Error>{
        let in_file = File::open(file_path)?;
        match serde_json::from_reader::<_, Self> (in_file) {
            Ok(model) => {Ok(model)}
            Err(_) => {Err(Error::new(ErrorKind::InvalidData, "Invalid model file."))}
        }
    }

    // exports the model to a .json file 
    pub fn export_model(&self, file_path: &str) -> Result<(), Error> {
        let out_file = OpenOptions::new().write(true).create(true).truncate(true).open(file_path)?;
        match serde_json::to_writer_pretty(out_file, &self) {
            Ok(_) => {Ok(())}
            Err(_) => {Err(Error::new(ErrorKind::Other, "Failed to export the model. Check your file path and permissions."))}
        }
    }

    // calculates the average x and y values and returns them as a single datapoint 
    fn calc_avg(dataset: &Vec<DataPoint>) -> DataPoint {
        // calculate our sums
        let mut sum_x: f64 = 0.0;
        let mut sum_y: f64 = 0.0;
        for point in dataset {
            sum_x += point.x;
            sum_y += point.y; 
        }
        // return the average datapoint
        let size = dataset.len() as f64;
        DataPoint { 
            x: sum_x / size,
            y: sum_y / size
        }
    }

    // calculates the slope of a given dataset
    fn calc_slope(&self, dataset: &Vec<DataPoint>, avg: &DataPoint) -> f64 {
        let mut numer: f64 = 0.0;
        let mut denom: f64 = 0.0;
        for point in dataset {
            let dev_x = point.x - avg.x;
            numer += dev_x * (point.y - avg.y);
            denom += dev_x * dev_x;
        }
        numer / denom
    }

    // calculates the y-intercept given the averages and slope of a dataset
    fn calc_intercept(avg: &DataPoint, slope: f64) -> f64 {
        avg.y - (avg.x*slope)
    }

    // trains the model from a file, this overwrites any current parameters
    pub fn train_file(&mut self, file_path: &str, fit_intercept: bool) -> Result<(),Error> {
        let dataset = import_dat(file_path)?;
        self.train(&dataset, fit_intercept);
        Ok(())
    }

    // trains the model using a given vector of DataPoints, this overwrites any current parameters
    pub fn train(&mut self, dataset: &Vec<DataPoint>, fit_intercept: bool) {
        self.avg = Self::calc_avg(dataset);
        self.slope = Self::calc_slope(&self, dataset, &self.avg);
        if fit_intercept {
            self.intercept = Self::calc_intercept(&self.avg, self.slope);
        }
    }

    // calculates a predicted y value for a given x value, train your model before using this.
    pub fn predict(&self, x: f64) -> f64 {
        (self.slope * x) + self.intercept
    } 
}

// reads all data points from a two-coulumn csv file
pub fn import_dat(file_path: &str) -> Result<Vec<DataPoint>, Error> {
    let mut out: Vec<DataPoint> = Vec::new();
    let in_file = File::open(file_path)?;
    let mut reader= csv::Reader::from_reader(in_file);
    for result in reader.deserialize() {
        let result: DataPoint = result?;
        out.push(result);
    }
    Ok(out)
} 
