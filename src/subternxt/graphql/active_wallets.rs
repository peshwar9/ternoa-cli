use graphql_client::{GraphQLQuery, Response};
use reqwest;
use std::error::Error;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/subternxt/graphql/schemas/mainnet_indexer_schema.json",
    query_path = "src/subternxt/graphql/queries/active_wallets.graphql",
    response_derives = "Debug"
)]
pub struct AccountEntities;

pub async fn get_active_wallets(url: String) -> Result<i64, Box<dyn Error>> {
    let request_body = AccountEntities::build_query(account_entities::Variables);

    let client = reqwest::Client::new();
    let res = client.post(url).json(&request_body).send().await?;
    let response_body: Response<account_entities::ResponseData> = res.json().await?;

    let total_count = if let Some(ref response_data) = response_body.data {
        if let Some(entities) = &response_data.account_entities {
            entities.total_count
        } else {
            0
        }
    } else {
        0
    };

    Ok(total_count)
}
