mod parse;

extern crate hyper;

use std::env;
use hyper::Client;
use crate::parse::nebc_to_maven;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Missing action argument
    if args.len() < 2 {
        panic!("Invalid command line arguments. Missing action.")
    }

    // Read action argument
    let action = &args[1];

    if action.eq("parse") && args.len() == 3 {
        let maven_artifact = nebc_to_maven(&args[2])
            .expect("Failed to parse nebc artifact");
        println!("{:?}\n", maven_artifact);
    }
}