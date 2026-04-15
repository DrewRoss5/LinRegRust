mod regression;
mod cli;
mod test;

use crate::cli::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let args_len = args.len();
    if args_len < 2 {
        print_err("This program expects at least one argument.");
        return;
    }
    // determine what operation to run based on the command
    let command = args[1].as_str();
    match command {
        "train" => {
            if args_len < 4 || args_len > 5{
                print_err("This command expects between two and three arguments.");
                return;
            }
            let fit_intercept = args_len == 5 && args[4] == "fit-intercept";
            train_model(args[2].as_str(), args[3].as_str(), fit_intercept);
        }
        "manual" => {
            if args_len != 3 {
                print_err("This command expects exactly one argument.");
                return;
            }
            manual_interface(args[2].as_str());
        }
        "auto" => {
            if args_len != 5 {
                print_err("This command expects exactly three arguments.");
                return;
            }
            auto_predict(args[2].as_str(), args[3].as_str(), args[4].as_str());
        }
        "test" => {
            let multithread: bool;
            match args_len {
                4 => {
                    multithread = false;
                }
                5 => {
                    match args[4].as_str() {
                        "multi" => {
                            multithread = true;
                        }
                        "single" => {
                            multithread = false;
                        }
                        _ => {
                            print_err("Threading argument must be \"multi\" or \"single\"");
                            return;
                        }
                    }
                }
                _ => {
                    print_err("This program expects between two and three arguments.");
                    return;
                }
            }
            test_model(args[2].as_str(), args[3].as_str(), multithread);
        }
        "help" => {
            print_help();
        }
        _ => {
            print_err("Unrecognized Command.");
        }
    }

}
