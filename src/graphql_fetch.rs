use derive_error::Error;
use graphql_client::{QueryBody, Response};
use reqwest::{self};
use serde::{Deserialize, Serialize};

#[derive(Debug, Error)]
pub(crate) enum GraphqlFetchError {
    ReqwestError(reqwest::Error),
}

pub(crate) fn graphql_fetch<T: for<'a> Deserialize<'a>, V: Serialize>(
    api_key: &str,
    query_body: &QueryBody<V>,
) -> Result<Response<T>, GraphqlFetchError> {
    let client = reqwest::blocking::Client::new();

    let response = client
        .post("https://api.linear.app/graphql")
        .header("Authorization", api_key)
        .json(query_body)
        .send()?
        .json()?;

    return Ok(response);
}
