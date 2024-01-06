extern crate derive_error;

mod config;
mod graphql_fetch;
mod issues;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "linear", version)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// list issues that are "In Progress" & "Todo"
    List,
    /// Move an issue to "In Progress"
    Claim { identifier: String },
}

fn main() {
    let config = config::get_config().expect("Unable to get config");
    let api_key = config.api_key;
    let team_name = config.team_name;

    let args = Args::parse();
    match args.command {
        Command::List => list_issues(&api_key, &team_name),
        Command::Claim { identifier } => claim_issue(&api_key, &team_name, &identifier),
    }
}

fn list_issues(api_key: &str, team_name: &str) {
    let issues = issues::fetch(api_key, team_name).expect("Failed to fetch issues");

    println!("In Progress");
    println!("===========");
    for issue in issues.in_progress {
        println!("[{}] {}", issue.identifier, issue.title);
    }

    println!("\n");
    println!("Todo");
    println!("====");
    for issue in issues.todo {
        println!("[{}] {}", issue.identifier, issue.title);
    }
}

fn claim_issue(api_key: &str, team_name: &str, identifier: &str) {
    issues::claim(api_key, team_name, identifier).expect("Failed to claim issue");
}
