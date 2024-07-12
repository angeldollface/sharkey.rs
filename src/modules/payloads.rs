/*
SHARKEY.RS by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the DSL v1.
*/

/// Importing the "Serialize"
/// trait to derive it for structures.
use serde::Serialize;

/// Importing the "NoteVisibility"
/// enum because this is required for making
/// new notes.
use super::enums::NoteVisibility;

/// Importing the "ReactionAcceptance"
/// enum because this is required for making
/// new notes.
use super::enums::ReactionAcceptance;

/// This structure holds
/// the JSON payload parameters
/// for a reacting to a user's note.
/// This structure can be used for the
/// following API routes:
/// - `/notes/reactions/create`
/// - `/notes/reactions/delete`
#[derive(Serialize)]
pub struct ReactionPayload {

    #[serde(rename(serialize = "noteId"))]
    pub note_id: String,
    pub reaction: String,
    pub i: String
}

/// This structure holds
/// the JSON payload parameters
/// for following a user.
/// This structure can be used for the
/// following API routes:
/// - `/following/create`
#[derive(Serialize)]
pub struct FollowPayload {

    #[serde(rename(serialize = "userId"))]
    pub user_id: String,

    #[serde(rename(serialize = "withReplies"))]
    pub with_replies: bool,

    pub i: String
}

/// This structure holds
/// the JSON payload parameters
/// for unfollowing a user.
/// This structure can be used for the
/// following API routes:
/// - `/following/delete`
#[derive(Serialize)]
pub struct UnfollowPayload {
    pub i: String,

    #[serde(rename(serialize = "userId"))]
    pub user_id: String

}

/// This structure holds
/// the JSON payload parameters
/// for deleting a note.
/// This structure can be used for the
/// following API routes:
/// - `/notes/delete`
#[derive(Serialize)]
pub struct DeleteNotePayload {
    #[serde(rename(serialize = "noteId"))]
    pub note_id: String,
    pub i: String
}

/// This structure holds
/// the JSON payload parameters
/// for creating a note.
/// This structure can be used for the
/// following API routes:
/// - `/notes/create`
#[derive(Serialize)]
pub struct CreateNotePayload {
    pub visibility: NoteVisibility,

    pub cw: Option<String>,

    #[serde(rename(serialize = "localOnly"))]
    pub local_only: bool,

    #[serde(rename(serialize = "reactionAcceptance"))]
    pub reaction_acceptance: Option<ReactionAcceptance>,

    #[serde(rename(serialize = "noExtractMentions"))]
    pub no_extract_mentions: bool,

    #[serde(rename(serialize = "noExtractHashtags"))]
    pub no_extract_hashtags: bool,

    #[serde(rename(serialize = "noExtractEmojis"))]
    pub no_extract_emojis: bool,

    #[serde(rename(serialize = "replyId"))]
    pub reply_id: Option<String>,

    #[serde(rename(serialize = "channelId"))]
    pub channel_id: Option<String>,
    pub text: String,
    pub i: String
}

/// This structure holds
/// the JSON payload parameters
/// for getting information on a
/// user. This structure can be
/// used for the following API routes:
/// - `/users/show`
#[derive(Serialize)]
pub struct UserInfoPayload {
    pub host: String,
    pub detailed: bool,
    pub username: String
}

/// This structure holds
/// the JSON payload parameters
/// for getting information on a
/// user's notes. This structure can be
/// used for the following API routes:
/// - `/users/notes`
#[derive(Serialize)]
pub struct UserNotesPayload {
    #[serde(rename(serialize = "userId"))]
    pub user_id: String,

    #[serde(rename(serialize = "withReplies"))]
    pub with_replies: bool,

    #[serde(rename(serialize = "withRenotes"))]
    pub with_renotes: bool,

    #[serde(rename(serialize = "withFiles"))]
    pub with_files: bool,

    #[serde(rename(serialize = "allowPartial"))]
    pub allow_partial: bool
}