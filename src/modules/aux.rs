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

/// Importing the structure for
/// maps in Rust.
use std::collections::HashMap;

/// Importing the "NoteVisibility"
/// enum for the "UserNote" structure.
use super::enums::NoteVisibility;

/// Importing the "SharkeyUser"
/// structure.
use super::responses::SharkeyUser;

/// Importing the "ReactionAcceptance"
/// enum for the "UserNote" structure.
use super::enums::ReactionAcceptance;

/// A structure to hold information
/// on a user's avatar decorations.
/// Needed for the `SharkeyUser`
/// structure from `./responses.rs`.
#[derive(Serialize, Deserialize)]
pub struct AvatarDecoration{
    pub id: String,
    pub url: String,
}

/// A structure to hold information
/// on a user's badge roles.
/// Needed for the `SharkeyUser`
/// structure from `./responses.rs`.
#[derive(Serialize, Deserialize)]
pub struct BadgeRole {
    pub name: String,

    #[serde(rename(serialize = "iconUrl"))]
    #[serde(rename(deserialize = "iconUrl"))]
    pub icon_url: Option<String>,

    #[serde(rename(serialize = "displayOrder"))]
    #[serde(rename(deserialize = "displayOrder"))]
    pub display_order: u32,
}

/// A structure to hold information
/// on a note a user has created.
/// Needed for the `CreatedNote`
/// structure from `./responses.rs`.
#[derive(Serialize, Deserialize)]
pub struct UserNote {
    pub id: String,

    #[serde(rename(serialize = "createdAt"))]
    #[serde(rename(deserialize = "createdAt"))]
    pub created_at: String,

    #[serde(rename(serialize = "userId"))]
    #[serde(rename(deserialize = "userId"))]
    pub user_id: String,

    pub user: SharkeyUser,

    pub text: Option<String>,
    pub cw: Option<String>,

    pub visibility: NoteVisibility,

    #[serde(rename(serialize = "localOnly"))]
    #[serde(rename(deserialize = "localOnly"))]
    pub local_only: bool,

    #[serde(rename(serialize = "reactionAcceptance"))]
    #[serde(rename(deserialize = "reactionAcceptance"))]
    pub reaction_acceptance: Option<ReactionAcceptance>,

    #[serde(rename(serialize = "renoteCount"))]
    #[serde(rename(deserialize = "renoteCount"))]
    pub renote_count: u32,

    #[serde(rename(serialize = "repliesCount"))]
    #[serde(rename(deserialize = "repliesCount"))]
    pub replies_count: u32,

    #[serde(rename(serialize = "reactionCount"))]
    #[serde(rename(deserialize = "reactionCount"))]
    pub reaction_count: u32,

    pub reactions: Option<HashMap<String, String>>,

    #[serde(rename(serialize = "reactionEmojis"))]
    #[serde(rename(deserialize = "reactionEmojis"))]
    pub reaction_emojis: Option<HashMap<String,String>>,

    pub file_ids: Option<Vec<String>>,

    pub files: Option<Vec<DriveFile>>,

    #[serde(rename(serialize = "replyId"))]
    #[serde(rename(deserialize = "replyId"))]
    pub reply_id: Option<String>,

    #[serde(rename(serialize = "renoteId"))]
    #[serde(rename(deserialize = "renoteId"))]
    pub renote_id: Option<String>,

    #[serde(rename(serialize = "clippedCount"))]
    #[serde(rename(deserialize = "clippedCount"))]
    pub clipped_count: u32,
}

#[derive(Serialize, Deserialize)]
pub struct DriveFile {
    pub id: String,

    #[serde(rename(serialize = "createdAt"))]
    #[serde(rename(deserialize = "createdAt"))]
    pub created_at: String,
    pub name: String,

    #[serde(rename(serialize = "type"))]
    #[serde(rename(deserialize = "type"))]
    pub file_type: String,
    pub md5: String,
    pub size: u32,

    #[serde(rename(serialize = "isSensitive"))]
    #[serde(rename(deserialize = "isSensitive"))]
    pub is_sensitive: bool,
    pub blurhash: Option<String>,
    pub properties: FileProperties,
    pub url: String,

    #[serde(rename(serialize = "thumbnailUrl"))]
    #[serde(rename(deserialize = "thumbnailUrl"))]
    pub thumbmnail_url: Option<String>,
    pub comment: Option<String>,

    #[serde(rename(serialize = "folderId"))]
    #[serde(rename(deserialize = "folderId"))]
    pub folder_id: Option<String>,
    pub folder: DriveFolder,

    #[serde(rename(serialize = "userId"))]
    #[serde(rename(deserialize = "userId"))]
    pub user_id: Option<String>,
    pub user: SharkeyUser,
}

#[derive(Serialize, Deserialize)]
pub struct DriveFolder {
    pub id: String,

    #[serde(rename(serialize = "createdAt"))]
    #[serde(rename(deserialize = "createdAt"))]
    pub created_at: String,
    pub name: String,

    #[serde(rename(serialize = "parentId"))]
    #[serde(rename(deserialize = "parentId"))]
    pub parent_id: String,

    #[serde(rename(serialize = "foldersCount"))]
    #[serde(rename(deserialize = "foldersCount"))]
    pub folders_count: String,

    #[serde(rename(serialize = "filesCount"))]
    #[serde(rename(deserialize = "filesCount"))]
    pub files_count: u32,
    pub parent: Box<Self>
}
#[derive(Serialize, Deserialize)]
pub struct FileProperties {
    pub width: u32,
    pub height: u32,
    pub orientation: u32,
    #[serde(rename(serialize = "avgColor"))]
    #[serde(rename(deserialize = "avgColor"))]
    pub avg_color: String
}
