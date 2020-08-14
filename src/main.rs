#[macro_use]
extern crate fstrings;

use clap::{App, load_yaml};
use json;
use git2::Repository;
use std::process::exit;
use std::path::Path;

fn main() {
    // Set up Clap
    let yaml_args = load_yaml!("arguments.yaml");
    let matches = App::from(yaml_args).get_matches();

    let platform = matches.value_of("PLATFORM").unwrap();
    let username = matches.value_of("USERNAME").unwrap();


    // Redefine path if set in arguments
    let mut clone_path = "./";
    if matches.is_present("path") {
        clone_path = matches.value_of("path").unwrap();
    } else {
        exit(0);
    }

    // Validate inputs
    assert!(platform == "github" || platform == "gitlab");  // Ensure a valid platform

    // Define variables
    let mut url = String::new();
    let mut page:i8 = 1;

    // Create Client
    let app_user_agent = concat!(
        env!("CARGO_PKG_NAME"),
        "/",
        env!("CARGO_PKG_VERSION")
    );
    let client = reqwest::blocking::Client::builder()
        .user_agent(app_user_agent)
        .build()
        .unwrap();

    // Clone profile
    loop {
        if platform == "github" {
            url = f!("https://api.github.com/users/{username}/repos?per_page=100&page={page}");
        } else if platform == "gitlab" {
            url = f!("https://gitlab.com/api/v4/users/{username}/projects?per_page=100&page={page}");
        }

        // Get data and parse into json
        let body = client.get(&url)
            .send()
            .unwrap()
            .text()
            .unwrap();

        let parsed = json::parse(&*body).unwrap();
        // Break if response is empty
        if parsed.is_empty() {
            break;
        }

        // Iterate through the results
        for i in 0..parsed.len() - 1 {
            // Get the clone url of the repository
            let mut repo_url = "";
            if platform == "github" {
                repo_url = parsed[i]["clone_url"].as_str().unwrap();
            } else if platform == "gitlab" {
                repo_url = parsed[i]["http_url_to_repo"].as_str().unwrap();
            }
            let repo_name = parsed[i]["name"].as_str().unwrap();
            let clone_path = format!("{}/{}", clone_path, repo_name);
            // Clone the repository
            println!("Cloning {}", repo_name);

            if !Path::new(&clone_path).exists() {
                let repo = match Repository::clone(repo_url, &clone_path) {
                    Ok(repo) => repo,
                    Err(E) => panic!("Failed to clone: {}", E),
                };
            } else {
                println!("The folder for {} already exists, moving on to next repository", repo_name);
            }
        }
        // Iterate to next page
        page += 1;
    }
}
