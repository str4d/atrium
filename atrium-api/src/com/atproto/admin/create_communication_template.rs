// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `com.atproto.admin.createCommunicationTemplate` namespace.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    ///Content of the template, markdown supported, can contain variable placeholders.
    pub content_markdown: String,
    ///DID of the user who is creating the template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<crate::types::string::Did>,
    ///Name of the template.
    pub name: String,
    ///Subject of the message, used in emails.
    pub subject: String,
}
pub type Output = crate::com::atproto::admin::defs::CommunicationTemplateView;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {}
