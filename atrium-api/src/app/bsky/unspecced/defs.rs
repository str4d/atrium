// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `app.bsky.unspecced.defs` namespace.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SkeletonSearchActor {
    pub did: crate::types::string::Did,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SkeletonSearchPost {
    pub uri: String,
}
