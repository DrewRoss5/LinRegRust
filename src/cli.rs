use crate::{regression::{self, DataPoint, Regression}, test::*};
use std::{fs::File, io::{BufRead, BufReader, Write, stdin, stdout}, sync::Arc, thread};

pub fn print_err(msg: &str) {
    eprintln!("\x1b[1m\x1b[31mError:\x1b[0m {} See https://github.com/DrewRoss5/LinRegRust for more information.", msg);
}

// trains and a linear model on the data in training_file and exports it to out_file
pub fn train_model(training_file: &str, out_file: &str, fit_intercept: bool) {
    let mut model = Regression::new();
    print!("Training Model...");
    stdout().flush().expect("Failed to flush output");
    match model.train_file(training_file, fit_intercept) {
        Ok(_) => {println!("OK!");}
        Err(err) => {
            eprintln!("\n{}", err);
            return;
        }
    }
    match model.export_model(out_file) {
        Ok(_) => {println!("Model stored to: {}.", out_file);}
        Err(_) => {print_err("Failed to store model. Check your file permissions.")}
    }
}

// imports a model from model_file and runs an interactive prompt to predict with it 
pub fn manual_interface(model_file: &str) {
    let model: Regression;
    match Regression::import_model(model_file) {
        Ok(reg) => {model = reg;}
        Err(e) => {
            print_err(e.to_string().as_str());
            return;
        }
    }
    println!("Enter a decimal x value to predict a y value, or enter \"quit\" to quit.");
    let mut buf = String::new();
    let mut x_str: &str;
    loop {
        // print prompt
        print!("> ");
        stdout().flush().expect("Failed to flush stdout");
        // read user's x value
        stdin().read_line(&mut buf).expect("Failed to read input");
        x_str = buf.trim();
        if x_str == "quit" {
            println!("Goodbye!");
            return;
        }
        match x_str.parse::<f64>() {
            Ok(x) => {println!("Y: {}", model.predict(x));}
            Err(_) => {println!("Please enter either \"quit\" or a number");}
        }
        buf.clear();
    }
}

pub fn auto_predict(model_file: &str, data_file: &str, out_file: &str) {
    let mut out: Vec<DataPoint> = Vec::new();
    let model: Regression;
    // import our model
    match Regression::import_model(model_file) {
        Ok(reg) => {model = reg;}
        Err(e) => {
            print_err(e.to_string().as_str());
            return;
        }
    }
    // read the input data
    let in_file: File;
    match File::open(data_file) {
        Ok(file) => {in_file = file;}
        Err(_) => {
            print_err("Failed to read input data file. Does it exist?");
            return;
        }
    }
    // generate our predictions
    print!("Predicting...");
    stdout().flush().expect("failed to flush stdout");
    let reader = BufReader::new(in_file);
    for line in reader.lines() {
        let x_str = line.expect("Failed to read line");
        match x_str.parse::<f64>() {
            Ok(x) => out.push(DataPoint { x, y: model.predict(x) }),
            Err(_) => {
                println!(""); // put a newline
                print_err("Invalid input data file.");
                return;
            }
        }
    }
    println!("OK");
    // write the line
    print!("Storing results...");
    stdout().flush().expect("failed to flush stdout");
    match csv::Writer::from_path(out_file) {
        Ok(mut wrt) => {
            for pred in out {
                wrt.serialize(pred).unwrap();
            }
        }
        Err(_) => {
            print_err("Failed to save data.");
            return;
        }
    }
    println!("OK.")
}

pub fn test_model(model_file: &str, data_file: &str, multithread: bool) {
    let model: Arc<Regression>;
    // import our model
    match Regression::import_model(model_file) {
        Ok(reg) => {model = Arc::new(reg);}
        Err(e) => {
            print_err(e.to_string().as_str());
            return;
        }
    }
    // read the input data
    let dataset: Arc<Vec<DataPoint>>;
    match regression::import_dat(data_file) {
        Ok(data) => {dataset = Arc::new(data)}
        Err(e) => {
            print_err(e.to_string().as_str());
            return;
        }
    }
    // run our benchmarks and display the results
    let ds_ptr1= Arc::clone(&dataset);
    let model_ptr1 = Arc::clone(&model);
    let ds_ptr2= Arc::clone(&dataset);
    let model_ptr2 = Arc::clone(&model);
    if multithread {
        let t1 = thread::spawn(move || {println!("Mean Absolute Error: {}", test_mae(model_ptr1, ds_ptr1))});
        let t2 = thread::spawn(move || {println!("Mean Squared Error: {}",  test_r2(model_ptr2, ds_ptr2))});
        t1.join().expect("Failed to join thread 1");
        t2.join().expect("Failed to join thread 2");
    }
    else {
        println!("Mean Absolute Error: {}", test_mae(model_ptr1, ds_ptr2));
        println!("Mean Squared Error: {}",  test_r2(model_ptr2, ds_ptr1))
    }
}

pub fn print_help() {
    let commands = vec!["\nCOMMAND:", "train", "manual", "auto", "help"];
    let args = vec!["ARGUMENTS:", "<train_file> <out_file> [fit-intercept]", "<model_file>", "<model_file> <data_file> <out_file>", ""];
    let descriptions = vec![
        "DESCRIPTION:",
        "Trains a model on train_file and stores it to out_file.",
        "Opens an interface to manually generate predictions with the model in model_file.",
        "Creates a prediction for each number in data_file, and stores them to out_file",
        "Displays this message.\n"
    ];
    for i in 0..5 {
        println!("{:<14}{:<45}{}", commands[i], args[i], descriptions[i])
    }
    println!("See https://github.com/DrewRoss5/LinRegRust for more information.\n")
}