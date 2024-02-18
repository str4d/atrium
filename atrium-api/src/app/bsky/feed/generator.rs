// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `app.bsky.feed.generator` namespace.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<crate::types::BlobRef>,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_facets: Option<Vec<crate::app::bsky::richtext::facet::Main>>,
    pub did: crate::types::string::Did,
    pub display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<RecordLabelsEnum>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum RecordLabelsEnum {
    #[serde(rename = "com.atproto.label.defs#selfLabels")]
    ComAtprotoLabelDefsSelfLabels(Box<crate::com::atproto::label::defs::SelfLabels>),
}
