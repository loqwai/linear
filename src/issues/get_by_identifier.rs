use derive_error::Error;
use graphql_client::{GraphQLQuery, Response};

use crate::graphql_fetch::{graphql_fetch, GraphqlFetchError};

use super::issue::Issue;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphql",
    query_path = "src/issues/get_by_identifier.graphql",
    response_derives = "Debug, Clone"
)]
pub(crate) struct IssueByIdentifierQuery;

#[derive(Debug, Error)]
pub(crate) enum GetByIdentifierError {
    GraphqlFetchError(GraphqlFetchError),
    NoData,
    IssueNotFound,
    MultipleIssuesFound,
    ParseFloatError(std::num::ParseFloatError),
}

pub(crate) fn get_by_identifier(
    api_key: &str,
    team_name: &str,
    identifier: &str,
) -> Result<Issue, GetByIdentifierError> {
    let issue_number: f64 = identifier
        .split_once('-')
        .unwrap_or(("", identifier))
        .1
        .parse()?;

    let request_body = IssueByIdentifierQuery::build_query(issue_by_identifier_query::Variables {
        team_name: team_name.to_string(),
        issue_number,
    });

    let response: Response<issue_by_identifier_query::ResponseData> =
        graphql_fetch(api_key, &request_body)?;

    let data = response.data.ok_or(GetByIdentifierError::NoData)?;
    let num_matched_issues = data.issues.nodes.len();
    let issue = match data.issues.nodes {
        _ if num_matched_issues == 0 => return Err(GetByIdentifierError::IssueNotFound),
        _ if num_matched_issues > 1 => return Err(GetByIdentifierError::MultipleIssuesFound),
        issues => issues[0].clone(),
    };

    return Ok(Issue {
        identifier: issue.identifier,
        title: issue.title,
        url: issue.url,
        sort_order: issue.sort_order,
        description: issue.description.unwrap_or("".to_string()),
    });
}
