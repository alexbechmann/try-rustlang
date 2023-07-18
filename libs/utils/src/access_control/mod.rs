// Code generated by jtd-codegen for Rust v0.2.1

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AccessControl {
    #[serde(rename = "vipe.access-control.access.assign.home")]
    VipeAccessControlAccessAssignHome(AccessControlVipeAccessControlAccessAssignHome),

    #[serde(rename = "vipe.access-control.access.assign.room")]
    VipeAccessControlAccessAssignRoom(AccessControlVipeAccessControlAccessAssignRoom),

    #[serde(rename = "vipe.access-control.access.delete.home")]
    VipeAccessControlAccessDeleteHome(AccessControlVipeAccessControlAccessDeleteHome),

    #[serde(rename = "vipe.access-control.access.delete.room")]
    VipeAccessControlAccessDeleteRoom(AccessControlVipeAccessControlAccessDeleteRoom),

    #[serde(rename = "vipe.access-control.access.expired")]
    VipeAccessControlAccessExpired(AccessControlVipeAccessControlAccessExpired),

    #[serde(rename = "vipe.access-control.access.replace.user")]
    VipeAccessControlAccessReplaceUser(AccessControlVipeAccessControlAccessReplaceUser),

    #[serde(rename = "vipe.access-control.access.revoke")]
    VipeAccessControlAccessRevoke(AccessControlVipeAccessControlAccessRevoke),

    #[serde(rename = "vipe.access-control.access.revoke.support")]
    VipeAccessControlAccessRevokeSupport(AccessControlVipeAccessControlAccessRevokeSupport),

    #[serde(rename = "vipe.access-control.access.revoke.user")]
    VipeAccessControlAccessRevokeUser(AccessControlVipeAccessControlAccessRevokeUser),
}

#[derive(Serialize, Deserialize)]
pub enum AccessControlVipeAccessControlAccessAssignHomeDataRole {
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

#[derive(Serialize, Deserialize)]
pub struct AccessControlVipeAccessControlAccessAssignHomeData {
    #[serde(rename = "homeUri")]
    pub homeUri: String,

    #[serde(rename = "role")]
    pub role: AccessControlVipeAccessControlAccessAssignHomeDataRole,

    #[serde(rename = "uri")]
    pub uri: String,

    #[serde(rename = "userId")]
    pub userId: String,

    #[serde(rename = "assignedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignedBy: Option<Box<String>>,

    #[serde(rename = "expiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiresAt: Option<Box<String>>,

    #[serde(rename = "supportId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supportId: Option<Box<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct AccessControlVipeAccessControlAccessAssignHome {
    #[serde(rename = "data")]
    pub data: AccessControlVipeAccessControlAccessAssignHomeData,

    #[serde(rename = "data_base64")]
    pub dataBase64: String,

    #[serde(rename = "datacontenttype")]
    pub datacontenttype: String,

    #[serde(rename = "dataschema")]
    pub dataschema: String,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "source")]
    pub source: String,

    #[serde(rename = "specversion")]
    pub specversion: String,

    #[serde(rename = "subject")]
    pub subject: String,

    #[serde(rename = "time")]
    pub time: DateTime<FixedOffset>,
}

#[derive(Serialize, Deserialize)]
pub enum AccessControlVipeAccessControlAccessAssignRoomDataRole {
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

#[derive(Serialize, Deserialize)]
pub struct AccessControlVipeAccessControlAccessAssignRoomData {
    #[serde(rename = "homeUri")]
    pub homeUri: String,

    #[serde(rename = "role")]
    pub role: AccessControlVipeAccessControlAccessAssignRoomDataRole,

    #[serde(rename = "roomUri")]
    pub roomUri: String,

    #[serde(rename = "uri")]
    pub uri: String,

    #[serde(rename = "userId")]
    pub userId: String,

    #[serde(rename = "assignedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignedBy: Option<Box<String>>,

    #[serde(rename = "expiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiresAt: Option<Box<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct AccessControlVipeAccessControlAccessAssignRoom {
    #[serde(rename = "data")]
    pub data: AccessControlVipeAccessControlAccessAssignRoomData,

    #[serde(rename = "data_base64")]
    pub dataBase64: String,

    #[serde(rename = "datacontenttype")]
    pub datacontenttype: String,

    #[serde(rename = "dataschema")]
    pub dataschema: String,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "source")]
    pub source: String,

    #[serde(rename = "specversion")]
    pub specversion: String,

    #[serde(rename = "subject")]
    pub subject: String,

    #[serde(rename = "time")]
    pub time: DateTime<FixedOffset>,
}

#[derive(Serialize, Deserialize)]
pub struct AccessControlVipeAccessControlAccessDeleteHomeData {
    #[serde(rename = "homeUri")]
    pub homeUri: String,
}

#[derive(Serialize, Deserialize)]
pub struct AccessControlVipeAccessControlAccessDeleteHome {
    #[serde(rename = "data")]
    pub data: AccessControlVipeAccessControlAccessDeleteHomeData,

    #[serde(rename = "data_base64")]
    pub dataBase64: String,

    #[serde(rename = "datacontenttype")]
    pub datacontenttype: String,

    #[serde(rename = "dataschema")]
    pub dataschema: String,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "source")]
    pub source: String,

    #[serde(rename = "specversion")]
    pub specversion: String,

    #[serde(rename = "subject")]
    pub subject: String,

    #[serde(rename = "time")]
    pub time: DateTime<FixedOffset>,
}

#[derive(Serialize, Deserialize)]
pub struct AccessControlVipeAccessControlAccessDeleteRoomData {
    #[serde(rename = "homeUri")]
    pub homeUri: String,

    #[serde(rename = "roomUri")]
    pub roomUri: String,
}

#[derive(Serialize, Deserialize)]
pub struct AccessControlVipeAccessControlAccessDeleteRoom {
    #[serde(rename = "data")]
    pub data: AccessControlVipeAccessControlAccessDeleteRoomData,

    #[serde(rename = "data_base64")]
    pub dataBase64: String,

    #[serde(rename = "datacontenttype")]
    pub datacontenttype: String,

    #[serde(rename = "dataschema")]
    pub dataschema: String,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "source")]
    pub source: String,

    #[serde(rename = "specversion")]
    pub specversion: String,

    #[serde(rename = "subject")]
    pub subject: String,

    #[serde(rename = "time")]
    pub time: DateTime<FixedOffset>,
}

#[derive(Serialize, Deserialize)]
pub struct AccessControlVipeAccessControlAccessExpiredData {
    #[serde(rename = "homeUri")]
    pub homeUri: String,

    #[serde(rename = "uri")]
    pub uri: String,
}

#[derive(Serialize, Deserialize)]
pub struct AccessControlVipeAccessControlAccessExpired {
    #[serde(rename = "data")]
    pub data: AccessControlVipeAccessControlAccessExpiredData,

    #[serde(rename = "data_base64")]
    pub dataBase64: String,

    #[serde(rename = "datacontenttype")]
    pub datacontenttype: String,

    #[serde(rename = "dataschema")]
    pub dataschema: String,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "source")]
    pub source: String,

    #[serde(rename = "specversion")]
    pub specversion: String,

    #[serde(rename = "subject")]
    pub subject: String,

    #[serde(rename = "time")]
    pub time: DateTime<FixedOffset>,
}

#[derive(Serialize, Deserialize)]
pub struct AccessControlVipeAccessControlAccessReplaceUserData {
    #[serde(rename = "homeUri")]
    pub homeUri: String,

    #[serde(rename = "newUserId")]
    pub newUserId: String,

    #[serde(rename = "oldUserId")]
    pub oldUserId: String,

    #[serde(rename = "expiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiresAt: Option<Box<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct AccessControlVipeAccessControlAccessReplaceUser {
    #[serde(rename = "data")]
    pub data: AccessControlVipeAccessControlAccessReplaceUserData,

    #[serde(rename = "data_base64")]
    pub dataBase64: String,

    #[serde(rename = "datacontenttype")]
    pub datacontenttype: String,

    #[serde(rename = "dataschema")]
    pub dataschema: String,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "source")]
    pub source: String,

    #[serde(rename = "specversion")]
    pub specversion: String,

    #[serde(rename = "subject")]
    pub subject: String,

    #[serde(rename = "time")]
    pub time: DateTime<FixedOffset>,
}

#[derive(Serialize, Deserialize)]
pub struct AccessControlVipeAccessControlAccessRevokeData {
    #[serde(rename = "homeUri")]
    pub homeUri: String,

    #[serde(rename = "uri")]
    pub uri: String,
}

#[derive(Serialize, Deserialize)]
pub struct AccessControlVipeAccessControlAccessRevoke {
    #[serde(rename = "data")]
    pub data: AccessControlVipeAccessControlAccessRevokeData,

    #[serde(rename = "data_base64")]
    pub dataBase64: String,

    #[serde(rename = "datacontenttype")]
    pub datacontenttype: String,

    #[serde(rename = "dataschema")]
    pub dataschema: String,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "source")]
    pub source: String,

    #[serde(rename = "specversion")]
    pub specversion: String,

    #[serde(rename = "subject")]
    pub subject: String,

    #[serde(rename = "time")]
    pub time: DateTime<FixedOffset>,
}

#[derive(Serialize, Deserialize)]
pub struct AccessControlVipeAccessControlAccessRevokeSupportData {
    #[serde(rename = "homeUri")]
    pub homeUri: String,

    #[serde(rename = "supportId")]
    pub supportId: String,
}

#[derive(Serialize, Deserialize)]
pub struct AccessControlVipeAccessControlAccessRevokeSupport {
    #[serde(rename = "data")]
    pub data: AccessControlVipeAccessControlAccessRevokeSupportData,

    #[serde(rename = "data_base64")]
    pub dataBase64: String,

    #[serde(rename = "datacontenttype")]
    pub datacontenttype: String,

    #[serde(rename = "dataschema")]
    pub dataschema: String,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "source")]
    pub source: String,

    #[serde(rename = "specversion")]
    pub specversion: String,

    #[serde(rename = "subject")]
    pub subject: String,

    #[serde(rename = "time")]
    pub time: DateTime<FixedOffset>,
}

#[derive(Serialize, Deserialize)]
pub struct AccessControlVipeAccessControlAccessRevokeUserData {
    #[serde(rename = "userId")]
    pub userId: String,

    #[serde(rename = "homeUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homeUri: Option<Box<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct AccessControlVipeAccessControlAccessRevokeUser {
    #[serde(rename = "data")]
    pub data: AccessControlVipeAccessControlAccessRevokeUserData,

    #[serde(rename = "data_base64")]
    pub dataBase64: String,

    #[serde(rename = "datacontenttype")]
    pub datacontenttype: String,

    #[serde(rename = "dataschema")]
    pub dataschema: String,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "source")]
    pub source: String,

    #[serde(rename = "specversion")]
    pub specversion: String,

    #[serde(rename = "subject")]
    pub subject: String,

    #[serde(rename = "time")]
    pub time: DateTime<FixedOffset>,
}
