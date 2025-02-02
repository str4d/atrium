// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `com.atproto.admin.sendEmail` namespace.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    ///Additional comment by the sender that won't be used in the email itself but helpful to provide more context for moderators/reviewers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    pub content: String,
    pub recipient_did: String,
    pub sender_did: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub sent: bool,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {}
