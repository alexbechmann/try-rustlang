// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

extern crate serde_derive;

/// CloudEvents Specification JSON Schema
#[derive(Serialize, Deserialize)]
pub struct AccessControl {
    /// Base64 encoded event payload. Must adhere to RFC4648.
    #[serde(rename = "data_base64")]
    pub data_base64: Option<String>,

    /// Content type of the data value. Must adhere to RFC 2046 format.
    #[serde(rename = "datacontenttype")]
    pub datacontenttype: Option<String>,

    /// Identifies the schema that data adheres to.
    #[serde(rename = "dataschema")]
    pub dataschema: Option<String>,

    /// Identifies the event.
    #[serde(rename = "id")]
    pub id: String,

    /// Identifies the context in which an event happened.
    #[serde(rename = "source")]
    pub source: String,

    /// The version of the CloudEvents specification which the event uses.
    #[serde(rename = "specversion")]
    pub specversion: String,

    /// Describes the subject of the event in the context of the event producer (identified by
    /// source).
    #[serde(rename = "subject")]
    pub subject: Option<String>,

    /// Timestamp of when the occurrence happened. Must adhere to RFC 3339.
    #[serde(rename = "time")]
    pub time: Option<String>,

    #[serde(rename = "type")]
    pub access_control_type: Type,

    #[serde(rename = "data")]
    pub data: Data,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "homeUri")]
    pub home_uri: Option<String>,

    #[serde(rename = "roomUri")]
    pub room_uri: Option<String>,

    #[serde(rename = "assignedBy")]
    pub assigned_by: Option<String>,

    #[serde(rename = "expiresAt")]
    pub expires_at: Option<String>,

    #[serde(rename = "role")]
    pub role: Option<AccessRole>,

    #[serde(rename = "supportId")]
    pub support_id: Option<String>,

    #[serde(rename = "uri")]
    pub uri: Option<String>,

    #[serde(rename = "userId")]
    pub user_id: Option<String>,

    #[serde(rename = "newUserId")]
    pub new_user_id: Option<String>,

    #[serde(rename = "oldUserId")]
    pub old_user_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "vipe.access-control.access.assign.home")]
    VipeAccessControlAccessAssignHome,

    #[serde(rename = "vipe.access-control.access.assign.room")]
    VipeAccessControlAccessAssignRoom,

    #[serde(rename = "vipe.access-control.access.delete.home")]
    VipeAccessControlAccessDeleteHome,

    #[serde(rename = "vipe.access-control.access.delete.room")]
    VipeAccessControlAccessDeleteRoom,

    #[serde(rename = "vipe.access-control.access.expired")]
    VipeAccessControlAccessExpired,

    #[serde(rename = "vipe.access-control.access.replace.user")]
    VipeAccessControlAccessReplaceUser,

    #[serde(rename = "vipe.access-control.access.revoke")]
    VipeAccessControlAccessRevoke,

    #[serde(rename = "vipe.access-control.access.revoke.support")]
    VipeAccessControlAccessRevokeSupport,

    #[serde(rename = "vipe.access-control.access.revoke.user")]
    VipeAccessControlAccessRevokeUser,
}

#[derive(Serialize, Deserialize)]
pub enum AccessRole {
    #[serde(rename = "INSTALLER")]
    Installer,

    #[serde(rename = "OWNER")]
    Owner,

    #[serde(rename = "READER")]
    Reader,

    #[serde(rename = "SUPPORTER")]
    Supporter,

    #[serde(rename = "TENANT")]
    Tenant,

    #[serde(rename = "USER")]
    User,
}
