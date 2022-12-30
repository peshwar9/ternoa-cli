#![allow(clippy::all, warnings)]
pub struct TotalTransactions;
pub mod total_transactions {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "TotalTransactions";
    pub const QUERY : & str = "query TotalTransactions {\n\nextrinsics(\n    filter: {\n      module: {\n        notEqualTo: \"timestamp\"\n      }\n    }\n  ) {\n    totalCount\n  }\n}" ;
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
        pub extrinsics: Option<TotalTransactionsExtrinsics>,
    }
    #[derive(Deserialize)]
    pub struct TotalTransactionsExtrinsics {
        #[serde(rename = "totalCount")]
        pub total_count: Int,
    }
}
impl graphql_client::GraphQLQuery for TotalTransactions {
    type Variables = total_transactions::Variables;
    type ResponseData = total_transactions::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: total_transactions::QUERY,
            operation_name: total_transactions::OPERATION_NAME,
        }
    }
}
