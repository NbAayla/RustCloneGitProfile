use std::path::Path;
use git2::Repository;

fn reqwest_client() -> reqwest::blocking::Client {
    let app_user_agent = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION")
    );
    let client = reqwest::blocking::Client::builder()
        .user_agent(app_user_agent)
        .build()
        .unwrap();
    return client;
}

fn clone_repository(url: &str, path: &str) {
    // Verify that the path does not already exist to prevent git from failing
    if !Path::new(path).exists() {
        // Path does not already exist. It is safe to clone.
        match Repository::clone(url, path) {
            Ok(repo) => repo,
            Err(e) => panic!("Failed to clone: {}", e),
        };
    } else {
        // Path already exists. Cloning will fail if attempted, so move on to the next repo.
        println!("Folder already exists. Not attempting to clone repository.");
    }
}

pub(crate) fn github(username: &str, path_argument: &str) {
    let mut page:i8 = 1;
    let client = reqwest_client();
    // Main loop
    loop {
        // Define the URL with an fstring
        let url = f!("https://api.github.com/users/{username}/repos?per_page=100&page={page}");
        // Send the request and unwrap down to the text
        let body = client.get(&url)
            .send()
            .unwrap()
            .text()
            .unwrap();
        // Parse the text as JSON
        let parsed = json::parse(&*body).unwrap();
        if parsed.is_empty() {
            /*
            Nothing will be returned if the page value is higher than what the user has repos to
            populate. The program is complete, and the main loop can be broken.
            */
            break
        } else {
            // Data was returned by the API. Iterate through it.
            for i in 0..parsed.len() {
                let repo_url = parsed[i]["clone_url"].as_str().unwrap();
                let repo_name = parsed[i]["name"].as_str().unwrap();
                let clone_path = &*format!("{}/{}", path_argument, repo_name);
                println_f!("Cloning {repo_name}");
                clone_repository(repo_url, clone_path);
            }
        }
        // Loop has completed. Move on to next page.
        page += 1;
    }
}

pub(crate) fn gitlab(username: &str, path_argument: &str) {
    assert!(false);
}