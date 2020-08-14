#[macro_use]
extern crate fstrings;

use clap::{App, load_yaml};
use json;
use git2::Repository;
use std::process::exit;

fn main() {
    let mut verbose = false;
    let mut platform = String::new();
    let mut username = String::new();
    let mut path = String::new();

    // Set up Clap
    let yaml_args = load_yaml!("arguments.yaml");
    let matches = App::from(yaml_args).get_matches();

    println!("{}", platform);
    assert!(platform == "github" || platform == "gitlab");  // Ensure a valid platform

    let mut url = String::new();
    let mut page:i8 = 1;
    loop {
        if platform == "github" {
            url = f!("https://api.github.com/users/{username}/repos?per_page=100&page={page}");
        } else if platform == "gitlab" {
            url = f!("https://gitlab.com/api/v4/users/{username}/projects?per_page=100&page={page}");
        }
        // Get data and parse into json
        let body = reqwest::blocking::get(&url)
            .unwrap()
            .text()
            .unwrap();
        let parsed = json::parse(&*body).unwrap();
        println!("{}", parsed);
        exit(0);
        // Iterate to next page
        page += 1;
    }

    let body = reqwest::blocking::get("https://afetzer.com/en.json")
        .unwrap()
        .text()
        .unwrap();
    println!("{}", body);
    let parsed = json::parse(&*body).unwrap();
    assert_eq!(parsed["Name"], "Aayla Fetzer");
}
