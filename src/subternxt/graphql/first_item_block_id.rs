use graphql_client::{GraphQLQuery, Response};
use reqwest::Client;
use std::error::Error;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/subternxt/graphql/schemas/mainnet_dictionary_schema.json",
    query_path = "src/subternxt/graphql/queries/first_item_block_id.graphql",
    response_derives = "Debug"
)]
pub struct GetFirstItemBlockId;

pub async fn get_first_item_block_id(url: String, event: String) -> Result<i64, Box<dyn Error>> {
    let client = Client::new();
    let request_body = GetFirstItemBlockId::build_query(get_first_item_block_id::Variables {
        event,
      });
    let response = client.post(url).json(&request_body).send().await?;
    let response_body: Response<get_first_item_block_id::ResponseData> = response.json().await?;

    let block_id = match response_body.data {
        Some(ref data) => {
            match data.events {
                Some(ref events) => {
                    match &events.nodes[0] {
                        Some(event) => event.block_id.parse::<i64>(),
                        None => Ok(0),
                    }
                }
                None => Ok(0),
            }
        }
        None => Ok(0),
    }?;
    Ok(block_id)
}
