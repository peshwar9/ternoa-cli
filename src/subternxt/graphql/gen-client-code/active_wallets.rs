#![allow(clippy::all, warnings)]
pub struct AccountEntities;
pub mod account_entities {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "AccountEntities";
    pub const QUERY: &str = "query AccountEntities {\n\naccountEntities { totalCount }\n}";
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Serialize)]
    pub struct Variables;
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "accountEntities")]
        pub account_entities: Option<AccountEntitiesAccountEntities>,
    }
    #[derive(Deserialize)]
    pub struct AccountEntitiesAccountEntities {
        #[serde(rename = "totalCount")]
        pub total_count: Int,
    }
}
impl graphql_client::GraphQLQuery for AccountEntities {
    type Variables = account_entities::Variables;
    type ResponseData = account_entities::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: account_entities::QUERY,
            operation_name: account_entities::OPERATION_NAME,
        }
    }
}
