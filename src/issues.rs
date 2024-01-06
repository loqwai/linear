use std::num::ParseFloatError;

use crate::graphql_fetch::{graphql_fetch, GraphqlFetchError};
use derive_error::Error;
use graphql_client::{GraphQLQuery, Response};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphql",
    query_path = "src/issues.graphql",
    response_derives = "Debug"
)]
pub(crate) struct IssuesQuery;

pub(crate) struct Issue {
    pub(crate) identifier: String,
    pub(crate) title: String,
}

impl From<&issues_query::IssuesQueryInProgressNodes> for Issue {
    fn from(value: &issues_query::IssuesQueryInProgressNodes) -> Self {
        Self {
            identifier: value.identifier.clone(),
            title: value.title.clone(),
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

pub(crate) fn fetch(api_key: &str, team_name: &str) -> Result<Issues, FetchError> {
    let request_body = IssuesQuery::build_query(issues_query::Variables {
        team_name: team_name.to_string(),
    });
    let response: Response<issues_query::ResponseData> = graphql_fetch(api_key, &request_body)?;
    let data = response.data.ok_or(FetchError::NoDataError)?;

    let in_progress: Vec<Issue> = data.in_progress.nodes.iter().map(Issue::from).collect();
    let todo: Vec<Issue> = data.todo.nodes.iter().map(Issue::from).collect();

    return Ok(Issues { in_progress, todo });
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphql",
    query_path = "src/get_claim_issue_ids.graphql",
    response_derives = "Debug"
)]

pub(crate) struct GetClaimIssueIdsQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphql",
    query_path = "src/claim_issue.graphql",
    response_derives = "Debug"
)]

pub(crate) struct ClaimIssueMutation;

#[derive(Debug, Error)]
pub(crate) enum ClaimError {
    GraphqlFetchError(GraphqlFetchError),
    MalformedIssueIdentifier,
    ParseFloatError(ParseFloatError),
    QueryIdsNoData,

    IssueNotFound,
    MultipleIssuesFound,

    StateNotFound,
    MultipleStatesFound,

    MutateIssueNoData,
    MutateIssueNotSuccessful,
}

pub(crate) fn claim(api_key: &str, team_name: &str, identifier: &str) -> Result<(), ClaimError> {
    let issue_number: f64 = identifier
        .split_once('-')
        .ok_or(ClaimError::MalformedIssueIdentifier)?
        .1
        .parse()?;

    let request_body = GetClaimIssueIdsQuery::build_query(get_claim_issue_ids_query::Variables {
        team_name: team_name.to_string(),
        issue_number,
    });

    let response: Response<get_claim_issue_ids_query::ResponseData> =
        graphql_fetch(api_key, &request_body)?;

    let data = response.data.ok_or(ClaimError::QueryIdsNoData)?;

    let num_matched_issues = data.issues.nodes.len();
    let issue_id = match data.issues.nodes {
        _ if num_matched_issues == 0 => return Err(ClaimError::IssueNotFound),
        _ if num_matched_issues > 1 => return Err(ClaimError::MultipleIssuesFound),
        issues => issues[0].id.to_string(),
    };

    let num_matched_states = data.states.nodes.len();
    let state_id = match data.states.nodes {
        _ if num_matched_states == 0 => return Err(ClaimError::StateNotFound),
        _ if num_matched_states > 1 => return Err(ClaimError::MultipleStatesFound),
        states => states[0].id.to_string(),
    };

    let user_id = data.viewer.id;

    let request_body = ClaimIssueMutation::build_query(claim_issue_mutation::Variables {
        issue_id,
        state_id,
        user_id,
    });
    let response: Response<claim_issue_mutation::ResponseData> =
        graphql_fetch(api_key, &request_body)?;

    let success = response
        .data
        .ok_or(ClaimError::MutateIssueNoData)?
        .issue_update
        .success;

    match success {
        false => Err(ClaimError::MutateIssueNotSuccessful),
        true => Ok(()),
    }
}
