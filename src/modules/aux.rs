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
    pub angle: u32,

    #[serde(rename(serialize = "flipH"))]
    #[serde(rename(deserialize = "flipH"))]
    pub flip_h: bool,
    pub url: String,

    #[serde(rename(serialize = "offsetX"))]
    #[serde(rename(deserialize = "offsetX"))]
    pub offset_x: u32,

    #[serde(rename(serialize = "offsetY"))]
    #[serde(rename(deserialize = "offsetY"))]
    pub offset_y: u32
}

/// A structure to hold information
/// on the instance a user is on.
/// Needed for the `SharkeyUser`
/// structure from `./responses.rs`.
#[derive(Serialize, Deserialize)]
pub struct FediInstance {
    pub name: Option<String>,

    #[serde(rename(serialize = "softwareName"))]
    #[serde(rename(deserialize = "softwareName"))]
    pub software_name: Option<String>,

    #[serde(rename(serialize = "softwareVersion"))]
    #[serde(rename(deserialize = "softwareVersion"))]
    pub software_version: Option<String>,

    #[serde(rename(serialize = "iconUrl"))]
    #[serde(rename(deserialize = "iconUrl"))]
    pub icon_url: Option<String>,

    #[serde(rename(serialize = "faviconUrl"))]
    #[serde(rename(deserialize = "faviconUrl"))]
    pub favicon_url: Option<String>,

    #[serde(rename(serialize = "themeColor"))]
    #[serde(rename(deserialize = "themeColor"))]
    pub theme_color: Option<String>
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
    pub behavior: String
}

/// A structure to hold information
/// on a poll created in a note.
/// Needed for the `UserNote`
/// structure from `./responses.rs`.
#[derive(Serialize, Deserialize)]
pub struct Poll {
    pub choices: Vec<String>,
    pub multiple: bool,

    #[serde(rename(serialize = "expiresAt"))]
    #[serde(rename(deserialize = "expiresAt"))]
    pub expires_at: Option<u32>,

    #[serde(rename(serialize = "expiredAfter"))]
    #[serde(rename(deserialize = "expiredAfter"))]
    pub expired_after: Option<u32>
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

    #[serde(rename(serialize = "deletedAt"))]
    #[serde(rename(deserialize = "deletedAt"))]
    pub deleted_at: Option<String>,
    pub text: Option<String>,
    pub cw: Option<String>,

    #[serde(rename(serialize = "userId"))]
    #[serde(rename(deserialize = "userId"))]
    pub user_id: String,
    pub user: SharkeyUser,

    #[serde(rename(serialize = "replyId"))]
    #[serde(rename(deserialize = "replyId"))]
    pub reply_id: Option<String>,

    #[serde(rename(serialize = "renoteId"))]
    #[serde(rename(deserialize = "renoteId"))]
    pub renote_id: Option<String>,

    #[serde(rename(serialize = "isHidden"))]
    #[serde(rename(deserialize = "isHidden"))]
    pub is_hidden: bool,
    pub visibility: NoteVisibility,
    pub mentions: Vec<String>,
    
    pub tags: Vec<String>,
    pub emojis: HashMap<String,String>,

    #[serde(rename(serialize = "channelId"))]
    #[serde(rename(deserialize = "channelId"))]
    pub channel_id: Option<String>,
    pub channel: Option<FediChannel>,

    #[serde(rename(serialize = "localOnly"))]
    #[serde(rename(deserialize = "localOnly"))]
    pub local_only: bool,

    #[serde(rename(serialize = "reactionAcceptance"))]
    #[serde(rename(deserialize = "reactionAcceptance"))]
    pub reaction_acceptance: Option<ReactionAcceptance>,

    #[serde(rename(serialize = "reactionEmojis"))]
    #[serde(rename(deserialize = "reactionEmojis"))]
    pub reaction_emojis: HashMap<String,String>,
    pub reactions: HashMap<String, String>,

    #[serde(rename(serialize = "reactionCount"))]
    #[serde(rename(deserialize = "reactionCount"))]
    pub reaction_count: u32,

    #[serde(rename(serialize = "renoteCount"))]
    #[serde(rename(deserialize = "renoteCount"))]
    pub renote_count: u32,

    #[serde(rename(serialize = "repliesCount"))]
    #[serde(rename(deserialize = "repliesCount"))]
    pub replies_count: u32,
    pub uri: String,
    pub url: String,

    #[serde(rename(serialize = "reactionAndUserPairCache"))]
    #[serde(rename(deserialize = "reactionAndUserPairCache"))]
    pub reaction_and_user_pair_cache: Vec<String>,

    #[serde(rename(serialize = "clippedCount"))]
    #[serde(rename(deserialize = "clippedCount"))]
    pub clipped_count: u32,

    #[serde(rename(serialize = "myReaction"))]
    #[serde(rename(deserialize = "myReaction"))]
    pub my_reaction: Option<String>
}

/// A structure to hold
/// information on
/// a Sharkey/Fediverse channel.
#[derive(Serialize, Deserialize)]
pub struct FediChannel {
    pub id: String,
    pub name: String,
    pub color: String,

    #[serde(rename(serialize = "isSensitive"))]
    #[serde(rename(deserialize = "isSensitive"))]
    pub is_sensitive: bool,

    #[serde(rename(serialize = "allowRenoteToExternal"))]
    #[serde(rename(deserialize = "allowRenoteToExternal"))]
    pub allow_renote_to_external: bool,

    #[serde(rename(serialize = "userId"))]
    #[serde(rename(deserialize = "userId"))]
    pub user_id: Option<String>
}