extern crate derive_error;

mod config;
mod graphql_fetch;
mod issues;

fn main() {
    let config = config::get_config().expect("Unable to get config");
    let api_key = config.api_key;
    let team_name = config.team_name;

    let issues = issues::fetch(&api_key, &team_name).expect("Failed to fetch issues");

    println!("\n");
    println!("In Progress");
    println!("===========");
    for issue in issues.in_progress {
        println!("{}: {}", issue.identifier, issue.title);
    }

    println!("\n");
    println!("Todo");
    println!("====");
    for issue in issues.todo {
        println!("{}: {}", issue.identifier, issue.title);
    }
}
