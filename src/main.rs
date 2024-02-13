extern crate derive_error;

mod config;
mod graphql_fetch;
mod issues;

use std::io::IsTerminal;

use clap::{Parser, Subcommand};
use config::{get_config, store_config};

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

    /// Move an issue to "In Progress", store it as current
    #[command(alias = "d")]
    Dibs { identifier: String },

    /// Actions for interacting with the current issue.
    #[command(alias = "c")]
    Current {
        #[clap(subcommand)]
        command: Option<CurrentCommand>,
    },

    /// Generate the markdown for any issue
    #[command(alias = "md")]
    Markdown { identifier: String },
}

#[derive(Debug, Subcommand)]
enum CurrentCommand {
    /// Show the current issue
    #[command(alias = "s")]
    Show,
    // /// Move the current issue to "Done"
    // Done,
    /// Move the current issue to "Blocked by Review"
    #[command(alias = "r")]
    Review,
    // /// Move the current issue to "In Progress"
    // Progress,
    // /// Move the current issue to "Todo"
    // Todo,
    /// Print the current issue's url
    #[command(alias = "u")]
    Url,

    /// Print a markdown link to the current issue
    #[command(alias = "md")]
    Markdown,
}

fn main() {
    let config = config::get_config().expect("Unable to get config");
    let api_key = config.api_key;
    let team_name = config.team_name;
    let current_issue_identifier = config.current_issue;

    let args = Args::parse();
    match args.command {
        Command::List => list_issues(&api_key, &team_name),
        Command::Dibs { identifier } => claim_issue(&api_key, &team_name, &identifier),
        Command::Current { command } => match command {
            Some(subcommand) => match subcommand {
                CurrentCommand::Show => {
                    current_issue(&api_key, &team_name, &current_issue_identifier)
                }
                CurrentCommand::Review => {
                    move_issue_to_review(&api_key, &team_name, &current_issue_identifier)
                }
                CurrentCommand::Url => {
                    print_current_issue_url(&api_key, &team_name, &current_issue_identifier)
                }
                CurrentCommand::Markdown => print_current_issue_markdown_url(
                    &api_key,
                    &team_name,
                    &current_issue_identifier,
                ),
            },
            None => current_issue(&api_key, &team_name, &current_issue_identifier),
        },
        Command::Markdown { identifier } => {
            print_current_issue_markdown_url(&api_key, &team_name, &Some(identifier))
        }
    }
}

fn list_issues(api_key: &str, team_name: &str) {
    let issues = issues::list(api_key, team_name).expect("Failed to fetch issues");

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
    let mut config = get_config().expect("Failed to get config for claim issue");

    issues::claim(api_key, team_name, identifier).expect("Failed to claim issue");
    config.current_issue = Some(identifier.to_string());
    store_config(&config).expect("Failed to store current issue in config");
}

fn current_issue(api_key: &str, team_name: &str, current_issue_identifier: &Option<String>) {
    let current_issue_identifier = current_issue_identifier.clone().expect("No current issue identifier in config.toml. Use `linear claim <identifier>` or set it in the config manually");

    let issue = issues::get_by_identifier(&api_key, &team_name, &current_issue_identifier)
        .expect("Failed to fetch current issue info");

    output_single_line(format!("[{}] {}", issue.identifier, issue.title))
}

fn move_issue_to_review(api_key: &str, team_name: &str, current_issue_identifier: &Option<String>) {
    let current_issue_identifier = current_issue_identifier.clone().expect("No current issue identifier in config.toml. Use `linear claim <identifier>` or set it in the config manually");

    issues::move_to_review(api_key, team_name, &current_issue_identifier)
        .expect("Failed to move issue to review");
}

fn print_current_issue_url(
    api_key: &str,
    team_name: &str,
    current_issue_identifier: &Option<String>,
) {
    let current_issue_identifier = current_issue_identifier.clone().expect("No current issue identifier in config.toml. Use `linear claim <identifier>` or set it in the config manually");

    let issue = issues::get_by_identifier(&api_key, &team_name, &current_issue_identifier)
        .expect("Failed to fetch current issue info");

    output_single_line(issue.url);
}

fn print_current_issue_markdown_url(
    api_key: &str,
    team_name: &str,
    current_issue_identifier: &Option<String>,
) {
    let current_issue_identifier = current_issue_identifier.clone().expect("No current issue identifier in config.toml. Use `linear claim <identifier>` or set it in the config manually");

    let issue = issues::get_by_identifier(&api_key, &team_name, &current_issue_identifier)
        .expect("Failed to fetch current issue info");

    output_single_line(format!(
        "[[{}] {}]({})",
        issue.identifier, issue.title, issue.url
    ));
}

/// Prints the line to stdout. If stdout is a terminal, appends a newline.
fn output_single_line(line: String) {
    if std::io::stdout().is_terminal() {
        println!("{}", line);
    } else {
        print!("{}", line);
    }
}
