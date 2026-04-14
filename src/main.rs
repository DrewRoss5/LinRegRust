mod regression;
mod cli;

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
            if args.len() < 4 || args.len() > 5{
                print_err("This command expects between two and three arguments.")
            }
            let fit_intercept = args.len() == 5 && args[4] == "fit-intercept";
            train_model(args[2].as_str(), args[3].as_str(), fit_intercept);
        }
        "manual" => {
            if args.len() != 3 {
                print_err("This command expects exactly one argument.");
                return;
            }
            manual_interface(args[2].as_str());
        }
        "auto" => {
            unimplemented!();
        }
        "help" => {
            unimplemented!();
        }
        _ => {
            eprintln!("Error: Unrecognized command. See https://github.com/DrewRoss5/LinRegRust for more information.")
        }
    }

}
