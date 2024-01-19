use crate::graphql_fetch::{graphql_fetch, GraphqlFetchError};
use derive_error::Error;
use graphql_client::{GraphQLQuery, Response};

use super::issue::Issue;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphql",
    query_path = "src/issues/list.graphql",
    response_derives = "Debug"
)]
pub(crate) struct IssuesQuery;

impl From<&issues_query::IssuesQueryInProgressNodes> for Issue {
    fn from(value: &issues_query::IssuesQueryInProgressNodes) -> Self {
        Self {
            identifier: value.identifier.clone(),
            title: value.title.clone(),
            url: value.url.clone(),
        }
    }
}

pub(crate) struct Issues {
    pub(crate) in_progress: Vec<Issue>,
    pub(crate) todo: Vec<Issue>,
}

#[derive(Debug, Error)]
pub(crate) enum FetchError {
    GraphqlFetchError(GraphqlFetchError),
    NoDataError,
}

pub(crate) fn list(api_key: &str, team_name: &str) -> Result<Issues, FetchError> {
    let request_body = IssuesQuery::build_query(issues_query::Variables {
        team_name: team_name.to_string(),
    });
    let response: Response<issues_query::ResponseData> = graphql_fetch(api_key, &request_body)?;
    let data = response.data.ok_or(FetchError::NoDataError)?;

    let in_progress: Vec<Issue> = data.in_progress.nodes.iter().map(Issue::from).collect();
    let todo: Vec<Issue> = data.todo.nodes.iter().map(Issue::from).collect();

    return Ok(Issues { in_progress, todo });
}
