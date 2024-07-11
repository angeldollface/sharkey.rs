/*
SHARKEY.RS by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the DSL v1.
*/

/// Importing the "Serialize"
/// trait to derive it for structures.
use serde::Serialize;

/// Importing the "Deserialize"
/// trait to derive it for structures.
use serde::Deserialize;

/// Importing the "UserNote" structure
/// for the "CreatedNote" structure.
use super::aux::UserNote;

/// Importing the "BadgeRole"
/// structure.
use super::aux::BadgeRole;

/// Importing the "FediInstance"
/// structure.
use super::aux::FediInstance;

/// Importing the "HashMap"
/// structure from the standard
/// library.
use std::collections::HashMap;

/// Importing the "OnlineStatus"
/// enum.
use super::enums::OnlineStatus;

/// Importing the "AvatarDecoration"
/// structure.
use super::aux::AvatarDecoration;

/// A structure to capture data
/// about a Sharkey user. This response
/// structure can be used for the following
/// Sharkey API routes:
/// - `/following/create`
/// - `/following/delete`
#[derive(Serialize, Deserialize)]
pub struct SharkeyUser {
    pub id: String,
    pub name: Option<String>,
    pub username: String,
    pub host: Option<String>,

    #[serde(rename(serialize = "avatarUrl"))]
    #[serde(rename(deserialize = "avatarUrl"))]
    pub avatar_url: Option<String>,

    #[serde(rename(serialize = "avatarBlurhash"))]
    #[serde(rename(deserialize = "avatarBlurhash"))]
    pub avatar_blurhash: Option<String>,

    #[serde(rename(serialize = "avatarDecorations"))]
    #[serde(rename(deserialize = "avatarDecorations"))]
    pub avatar_decorations: Vec<AvatarDecoration>,

    #[serde(rename(serialize = "isBot"))]
    #[serde(rename(deserialize = "isBot"))]
    pub is_bot: bool,

    #[serde(rename(serialize = "isCat"))]
    #[serde(rename(deserialize = "isCat"))]
    pub is_cat: bool,
    pub instance: FediInstance,
    pub emojis: HashMap<String, String>,

    #[serde(rename(serialize = "onlineStatus"))]
    #[serde(rename(deserialize = "onlineStatus"))]
    pub online_status: OnlineStatus,

    #[serde(rename(serialize = "badgeRoles"))]
    #[serde(rename(deserialize = "badgeRoles"))]
    pub badge_roles: Vec<BadgeRole>
}

/// A structure to reflect the status of an
/// operation that does not return
/// a JSON response. This response
/// structure can be used for the following
/// Sharkey API routes:
/// - `/notes/reactions/create`
/// - `/notes/reactions/delete`
/// - `/notes/delete`
#[derive(Serialize, Deserialize)]
pub struct OperationStatus {
    pub status: u32,
    pub success: u32
}

/// A structure to return the note
/// a user has created. This response structure
/// can be used for the following Sharkey API routes:
/// - `/notes/create`
#[derive(Serialize, Deserialize)]
pub struct CreatedNote {
    #[serde(rename(serialize = "createdNote"))]
    #[serde(rename(deserialize = "createdNote"))]
    pub created_note: UserNote
}