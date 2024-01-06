extern crate derive_error;

mod config;
mod graphql_fetch;
mod issues;

fn main() {
    let config = config::get_config().expect("Unable to get config");
    let api_key = config.api_key;
    let team_name = config.team_name;

    let issues = issues::fetch(&api_key, &team_name).expect("Failed to fetch issues");

    for issue in issues {
        println!("{}: {}", issue.identifier, issue.title);
    }
}
