extern crate derive_error;

use derive_error::Error;
use std::{
    env::{self},
    fs,
};

use graphql_client::{GraphQLQuery, QueryBody, Response};
use reqwest::{self};
use serde::{Deserialize, Serialize};

fn main() {
    let config = get_config().expect("Unable to get config");

    let response =
        fetch_issues(&config.api_key, &config.team_name).expect("Failed to fetch issues");
    let data = response.data.expect("Did not receive data");

    for edge in data.issues.edges {
        let issue = edge.node;
        println!("{}: {}", issue.identifier, issue.title);
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphql",
    query_path = "src/issues.graphql",
    response_derives = "Debug"
)]
pub struct IssuesQuery;

#[derive(Debug, Error)]
enum FetchError {
    ReqwestError(reqwest::Error),
}

fn fetch<T: for<'a> Deserialize<'a>, V: Serialize>(
    api_key: &str,
    query_body: &QueryBody<V>,
) -> Result<Response<T>, FetchError> {
    let client = reqwest::blocking::Client::new();

    let response = client
        .post("https://api.linear.app/graphql")
        .header("Authorization", api_key)
        .json(query_body)
        .send()?
        .json()?;

    return Ok(response);
}

fn fetch_issues(
    api_key: &str,
    team_name: &str,
) -> Result<Response<issues_query::ResponseData>, FetchError> {
    let request_body = IssuesQuery::build_query(issues_query::Variables {
        team_name: team_name.to_string(),
    });
    let response: Response<issues_query::ResponseData> = fetch(api_key, &request_body)?;

    return Ok(response);
}

#[derive(Deserialize, Debug)]
struct Config {
    api_key: String,
    team_name: String,
}

#[derive(Debug, Error)]
enum ConfigError {
    EnvVarError(env::VarError),
    FileReadError(std::io::Error),
    ParseError(toml::de::Error),
}

fn get_config() -> Result<Config, ConfigError> {
    let home = env::var("HOME")?;
    let file_path = format!("{}/.config/linear/config.toml", home);

    let config_str = fs::read_to_string(file_path)?;
    let config: Config = toml::from_str(&config_str)?;

    return Ok(config);
}
