// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `com.atproto.sync.getBlob` namespace.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    ///The CID of the blob to fetch
    pub cid: String,
    ///The DID of the repo.
    pub did: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {}
