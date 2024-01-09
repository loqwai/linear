use crate::graphql_fetch::{graphql_fetch, GraphqlFetchError};
use derive_error::Error;
use graphql_client::{GraphQLQuery, Response};

use super::get_move_issue_ids::{get_move_issue_ids, GetMoveIssueIdsError};

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
    GetClaimIdsError(GetMoveIssueIdsError),
    MutateIssueNoData,
    MutateIssueNotSuccessful,
}

pub(crate) fn claim(api_key: &str, team_name: &str, identifier: &str) -> Result<(), ClaimError> {
    let claim_ids = get_move_issue_ids(api_key, team_name, identifier)?;

    let request_body = ClaimIssueMutation::build_query(claim_issue_mutation::Variables {
        issue_id: claim_ids.issue_id,
        state_id: claim_ids.in_progress_state_id,
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
