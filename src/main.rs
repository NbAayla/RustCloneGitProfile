mod clone;

#[macro_use]
extern crate fstrings;

use clap::{App, load_yaml};
use std::process::exit;

fn main() {
    // Set up Clap
    let yaml_args = load_yaml!("arguments.yaml");
    let matches = App::from(yaml_args).get_matches();

    // Clone profile
    match matches.subcommand_name() {
        Some("github") => {
            // Get independent subcommand matches
            if let Some(matches) = matches.subcommand_matches("github") {
                println!("Cloning GitHub profile of {}", matches.value_of("USERNAME").unwrap());
                // Clone the profile
                clone::github(matches.value_of("USERNAME").unwrap(),
                              matches.value_of("path").unwrap(),
                              matches.values_of("languages").unwrap()  // clap::Values
                );
            }
        }
        Some("gitlab") => {
            // Get independent subcommand matches
            if let Some(matches) = matches.subcommand_matches("gitlab") {
                clone::gitlab(matches.value_of("USERNAME").unwrap(),
                              matches.value_of("path").unwrap());
            }
        }
            _ => {
                println!("Command missing! Try with -h for more info.");
                exit(1);
            }
        }
}
