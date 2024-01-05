use std::fs;

use graphql_client::{GraphQLQuery, Response};
use reqwest::{self};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    api_key: String,
}

fn main() {
    let config_str = fs::read_to_string("config.toml").expect("Unable to read file config.toml");
    let config: Config = toml::from_str(&config_str).expect("Unable to parse config.toml");

    fetch_issues(&config.api_key)
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphql",
    query_path = "src/issues.graphql",
    response_derives = "Debug"
)]
pub struct IssuesQuery;

fn fetch_issues(api_key: &str) {
    let request_body = IssuesQuery::build_query(issues_query::Variables {});
    let client = reqwest::blocking::Client::new();
    let res = client
        .post("https://api.linear.app/graphql")
        .header("Authorization", api_key)
        .json(&request_body)
        .send()
        .expect("Request failed");

    let response_body: Response<issues_query::ResponseData> = res.json().expect("Unable to parse");

    println!("data: {:?}", response_body.data);
}
