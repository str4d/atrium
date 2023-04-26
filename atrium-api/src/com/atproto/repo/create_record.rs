// This file is generated by atrium-codegen. Do not edit.
//! Definitions for the `com.atproto.repo.createRecord` namespace.

/// Create a new record.
#[async_trait::async_trait]
pub trait CreateRecord: crate::xrpc::XrpcClient {
    async fn create_record(&self, input: Input) -> Result<Output, Box<dyn std::error::Error>> {
        crate::xrpc::XrpcClient::send(
            self,
            http::Method::POST,
            "com.atproto.repo.createRecord",
            Option::<()>::None,
            Some(input),
        )
        .await
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    /// The NSID of the record collection.
    pub collection: String,
    /// The record to create.
    pub record: crate::records::Record,
    /// The handle or DID of the repo.
    pub repo: String,
    /// The key of the record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rkey: Option<String>,
    /// Compare and swap with the previous commit by cid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap_commit: Option<String>,
    /// Validate the record?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate: Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub cid: String,
    pub uri: String,
}

pub enum Error {
    InvalidSwap,
}