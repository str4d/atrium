// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `app.bsky.graph.getRelationships` namespace.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    pub actor: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub others: Option<Vec<String>>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<String>,
    pub relationships: Vec<OutputRelationshipsItem>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {
    ///the primary actor at-identifier could not be resolved
    ActorNotFound(Option<String>),
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum OutputRelationshipsItem {
    #[serde(rename = "app.bsky.graph.defs#relationship")]
    AppBskyGraphDefsRelationship(Box<crate::app::bsky::graph::defs::Relationship>),
    #[serde(rename = "app.bsky.graph.defs#notFoundActor")]
    AppBskyGraphDefsNotFoundActor(Box<crate::app::bsky::graph::defs::NotFoundActor>),
}
