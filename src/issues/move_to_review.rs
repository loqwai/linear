use derive_error::Error;
use graphql_client::{GraphQLQuery, Response};

use crate::graphql_fetch::{graphql_fetch, GraphqlFetchError};

use super::get_move_issue_ids::{get_move_issue_ids, GetMoveIssueIdsError};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphql",
    query_path = "src/issues/move_to_review.graphql",
    response_derives = "Debug"
)]
struct MoveToReviewMutation;

#[derive(Debug, Error)]
pub(crate) enum MoveToReviewError {
    GetMoveIssueIdsError(GetMoveIssueIdsError),
    GraphqlFetchError(GraphqlFetchError),
    NoData,
    FailedToMoveIssue,
    NoBlockedByReviewState,
}

pub(crate) fn move_to_review(
    api_key: &str,
    team_name: &str,
    identifier: &str,
) -> Result<(), MoveToReviewError> {
    let ids = get_move_issue_ids(api_key, team_name, identifier)?;

    let request_body = MoveToReviewMutation::build_query(move_to_review_mutation::Variables {
        issue_id: ids.issue_id,
        state_id: ids
            .blocked_by_review_state_id
            .ok_or(MoveToReviewError::NoBlockedByReviewState)?,
    });

    let response: Response<move_to_review_mutation::ResponseData> =
        graphql_fetch(api_key, &request_body)?;

    let success = response
        .data
        .ok_or(MoveToReviewError::NoData)?
        .issue_update
        .success;

    match success {
        false => Err(MoveToReviewError::FailedToMoveIssue),
        true => Ok(()),
    }
}
