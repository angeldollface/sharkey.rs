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

/// Importing the "HashMap"
/// structure from the standard
/// library.
use std::collections::HashMap;

/// Importing the "OnlineStatus"
/// enum.
use super::enums::OnlineStatus;

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

    pub description: Option<String>,

    #[serde(rename(serialize = "createdAt"))]
    #[serde(rename(deserialize = "createdAt"))]
    pub created_at: String,

    #[serde(rename(serialize = "isBot"))]
    #[serde(rename(deserialize = "isBot"))]
    pub is_bot: bool,

    #[serde(rename(serialize = "isCat"))]
    #[serde(rename(deserialize = "isCat"))]
    pub is_cat: bool,
    pub noindex: bool,
    #[serde(rename(serialize = "isSilenced"))]
    #[serde(rename(deserialize = "isSilenced"))]
    pub is_silenced: bool,

    #[serde(rename(serialize = "speakAsCat"))]
    #[serde(rename(deserialize = "speakAsCat"))]
    pub speak_as_cat: bool,
    pub approved: bool,

    #[serde(rename(serialize = "followersCount"))]
    #[serde(rename(deserialize = "followersCount"))]
    pub followers_count: u32,

    #[serde(rename(serialize = "followingCount"))]
    #[serde(rename(deserialize = "followingCount"))]
    pub following_count: u32,
    pub emojis: Option<HashMap<String, String>>,

    #[serde(rename(serialize = "notesCount"))]
    #[serde(rename(deserialize = "notesCount"))]
    pub note_count: u32,

    #[serde(rename(serialize = "onlineStatus"))]
    #[serde(rename(deserialize = "onlineStatus"))]
    pub online_status: OnlineStatus,

    #[serde(rename(serialize = "badgeRoles"))]
    #[serde(rename(deserialize = "badgeRoles"))]
    pub badge_roles: Option<Vec<BadgeRole>>
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