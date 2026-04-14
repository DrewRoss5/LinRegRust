# LinRegRust
A simple Linear Regression implementation in Rust that implements model persistence. Currently, this only support single-feature regression, however that may change in a later version. 

## Getting Started:
To compile this and program:
- Clone this repository.
- Ensure that [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) is installed.
- Run `cargo build --release` from this repository's root directory.
- The compiled application can be locataed at `/path/to/LinRegRust/release/linreg`

## Using This Program:
Currently, there are three primary commands available.
- ### Train a Model
  - **Syntax:** `linreg train <training_file> <model_file_path> [fit-intercept]`
  - **Description:**
    - This trains a model on the data in `training_file` and exports it to <model_file_path> as a json file. The training file should be a headered CSV file with two columns holding only floating point values, the first coulumn represents your predicting variable, and the second represents your predicted variable.
  - 
- ### Manually Generate Predictions
  - **Syntax:**  `linreg manual <model_file>`
  - **Description:**
    - This opens an interface to manually enter input variables and view the model's predictions for their outputs. The `model_file` is expected to be a json file created with the `train` command.
   
- ### Predict Outputs for an Input File:
  - **Syntax:**: `linreg auto <model_file> <data_file> <out_file>`
  - **Description**:
    - This uses the model stored in `model_file` to create predictions for all values in `data_file`, and exports the inputs and their predictions to `out_file` in CSV format. The `data_file` should be a plaintext file containing only floating point numbers seperated by newlines. 
