use miner::dataframe::timeseriesbatch::TimeSeriesBatch;
use miner::miner::bt;
use miner::strategy::strategy::SimpleSMA;

use std::env;

#[derive(Debug)]
struct Config {
    root_directory: String,
    gpu_enable: bool,
}

impl Config {
    pub fn new() -> Config {
        let mut root_directory: Option<String> = None;
        let mut gpu_enable = false;

        let mut args = env::args().skip(1);
        while let Some(arg) = args.next() {
            match &arg[..] {
                "-h" | "--help" => Self::help(),
                "-a" | "--accelerate" => gpu_enable = true,
                "-d" | "--data_dir" => {
                    if let Some(path) = args.next() {
                        root_directory = Some(path);
                    } else {
                        panic!("No value specified for the parameter")
                    }
                }
                _ => {
                    if arg.starts_with('-') {
                        println!("Unknown arguments {}", arg);
                    } else {
                        println!("Uknown positional argument {}", arg);
                    }
                }
            }
        }

        if let Some(root_directory) = root_directory {
            Config {
                root_directory,
                gpu_enable,
            }
        } else {
            panic!("No input directory was provided. Use the -d argument.");
        }
    }

    pub fn help() {
        println!("Welcome to miner!");
    }
}


fn main() {
    let config = Config::new();
    println!("{:?}", config);


    let batch = TimeSeriesBatch::load_from_folder(&config.root_directory);
    for stock in batch.data {
        bt::run(stock, SimpleSMA::new(), 10000.0, 0.02);

    }
    // for mut stock in batch.data {
    //     for ticker in stock {
    //         println!("{:?}", ticker);
    //     }
    // }
    //     println!("{:?}", stock.buffer.next());
    //     println!("{:?}", stock.buffer.next());
    //     println!("{:?}", stock.buffer.next());
    //     println!("{:?}", stock.buffer.next());
        // for line in stock.buffer {
        //     println!("{:?}", line);
        // }
    // } 
    // let results = bt::run_batch(&batch);
    // println!("{:?}", batch);
}