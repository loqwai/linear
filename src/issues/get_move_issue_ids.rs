use std::num::ParseFloatError;

use crate::graphql_fetch::{graphql_fetch, GraphqlFetchError};
use derive_error::Error;
use graphql_client::{GraphQLQuery, Response};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphql",
    query_path = "src/issues/get_move_issue_ids.graphql",
    response_derives = "Debug"
)]

struct GetClaimIssueIdsQuery;

pub(crate) struct MoveIssueIds {
    pub(crate) issue_id: String,
    pub(crate) in_progress_state_id: String,
    pub(crate) blocked_by_review_state_id: String,
    pub(crate) user_id: String,
}

#[derive(Debug, Error)]
pub(crate) enum GetMoveIssueIdsError {
    GraphqlFetchError(GraphqlFetchError),
    MalformedIssueIdentifier,
    ParseFloatError(ParseFloatError),
    QueryIdsNoData,

    IssueNotFound,
    MultipleIssuesFound,

    StateNotFound,
    MultipleStatesFound,
}

pub(crate) fn get_move_issue_ids(
    api_key: &str,
    team_name: &str,
    identifier: &str,
) -> Result<MoveIssueIds, GetMoveIssueIdsError> {
    let issue_number: f64 = identifier
        .split_once('-')
        .ok_or(GetMoveIssueIdsError::MalformedIssueIdentifier)?
        .1
        .parse()?;

    let request_body = GetClaimIssueIdsQuery::build_query(get_claim_issue_ids_query::Variables {
        team_name: team_name.to_string(),
        issue_number,
    });

    let response: Response<get_claim_issue_ids_query::ResponseData> =
        graphql_fetch(api_key, &request_body)?;

    let data = response.data.ok_or(GetMoveIssueIdsError::QueryIdsNoData)?;

    let num_matched_issues = data.issues.nodes.len();
    let issue_id = match data.issues.nodes {
        _ if num_matched_issues == 0 => return Err(GetMoveIssueIdsError::IssueNotFound),
        _ if num_matched_issues > 1 => return Err(GetMoveIssueIdsError::MultipleIssuesFound),
        issues => issues[0].id.to_string(),
    };

    let num_matched_states = data.in_progress_states.nodes.len();
    let in_progress_state_id = match data.in_progress_states.nodes {
        _ if num_matched_states == 0 => return Err(GetMoveIssueIdsError::StateNotFound),
        _ if num_matched_states > 1 => return Err(GetMoveIssueIdsError::MultipleStatesFound),
        states => states[0].id.to_string(),
    };

    let num_matched_states = data.blocked_by_review_states.nodes.len();
    let blocked_by_review_state_id = match data.blocked_by_review_states.nodes {
        _ if num_matched_states == 0 => return Err(GetMoveIssueIdsError::StateNotFound),
        _ if num_matched_states > 1 => return Err(GetMoveIssueIdsError::MultipleStatesFound),
        states => states[0].id.to_string(),
    };

    let user_id = data.viewer.id;

    return Ok(MoveIssueIds {
        issue_id,
        in_progress_state_id,
        blocked_by_review_state_id,
        user_id,
    });
}
