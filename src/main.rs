use std::io::{self, BufRead};

use std::env;
use std::fs;

use atty::Stream;

fn main() {
    let mut read_from_pipe = false;
    let mut expected_args = 3;
    if !atty::is(Stream::Stdin) {
        read_from_pipe = true;
        expected_args = 2;
    }

    if env::args().len() != expected_args {
        println!("error: not enough arguments");
        std::process::exit(1);
    }

    let pattern = env::args().nth(1).unwrap();

    if !pattern.contains(":") {
        println!("error: pattern is malformed");
        std::process::exit(2);
    }

    let parts = pattern.split(":");
    let parts_vec: Vec<&str> = parts.collect();

    let expr = parts_vec[0];
    let replace = parts_vec[1];

    if read_from_pipe {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let line = line.expect("error: could not read line from standard in");
            println!("{}", line.replace(expr, replace));
        }
        std::process::exit(0);
    }

    let file = env::args().nth(2).unwrap();
    let file_content = fs::read_to_string(file)
        .expect("error: cannot read the file");
    
    println!("{}", file_content.replace(expr, replace));
}

