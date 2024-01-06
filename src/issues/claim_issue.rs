use std::num::ParseFloatError;

use crate::graphql_fetch::{graphql_fetch, GraphqlFetchError};
use derive_error::Error;
use graphql_client::{GraphQLQuery, Response};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphql",
    query_path = "src/issues/get_claim_ids.graphql",
    response_derives = "Debug"
)]

pub(crate) struct GetClaimIssueIdsQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphql",
    query_path = "src/issues/claim.graphql",
    response_derives = "Debug"
)]

pub(crate) struct ClaimIssueMutation;

#[derive(Debug, Error)]
pub(crate) enum ClaimError {
    GraphqlFetchError(GraphqlFetchError),
    GetClaimIdsError(GetClaimIdsError),
    MutateIssueNoData,
    MutateIssueNotSuccessful,
}

pub(crate) fn claim(api_key: &str, team_name: &str, identifier: &str) -> Result<(), ClaimError> {
    let claim_ids = get_claim_ids(api_key, team_name, identifier)?;

    let request_body = ClaimIssueMutation::build_query(claim_issue_mutation::Variables {
        issue_id: claim_ids.issue_id,
        state_id: claim_ids.state_id,
        user_id: claim_ids.user_id,
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

struct ClaimIds {
    issue_id: String,
    state_id: String,
    user_id: String,
}

#[derive(Debug, Error)]
pub(crate) enum GetClaimIdsError {
    GraphqlFetchError(GraphqlFetchError),
    MalformedIssueIdentifier,
    ParseFloatError(ParseFloatError),
    QueryIdsNoData,

    IssueNotFound,
    MultipleIssuesFound,

    StateNotFound,
    MultipleStatesFound,
}

fn get_claim_ids(
    api_key: &str,
    team_name: &str,
    identifier: &str,
) -> Result<ClaimIds, GetClaimIdsError> {
    let issue_number: f64 = identifier
        .split_once('-')
        .ok_or(GetClaimIdsError::MalformedIssueIdentifier)?
        .1
        .parse()?;

    let request_body = GetClaimIssueIdsQuery::build_query(get_claim_issue_ids_query::Variables {
        team_name: team_name.to_string(),
        issue_number,
    });

    let response: Response<get_claim_issue_ids_query::ResponseData> =
        graphql_fetch(api_key, &request_body)?;

    let data = response.data.ok_or(GetClaimIdsError::QueryIdsNoData)?;

    let num_matched_issues = data.issues.nodes.len();
    let issue_id = match data.issues.nodes {
        _ if num_matched_issues == 0 => return Err(GetClaimIdsError::IssueNotFound),
        _ if num_matched_issues > 1 => return Err(GetClaimIdsError::MultipleIssuesFound),
        issues => issues[0].id.to_string(),
    };

    let num_matched_states = data.states.nodes.len();
    let state_id = match data.states.nodes {
        _ if num_matched_states == 0 => return Err(GetClaimIdsError::StateNotFound),
        _ if num_matched_states > 1 => return Err(GetClaimIdsError::MultipleStatesFound),
        states => states[0].id.to_string(),
    };

    let user_id = data.viewer.id;

    return Ok(ClaimIds {
        issue_id,
        state_id,
        user_id,
    });
}
