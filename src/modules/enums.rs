/*
SHARKEY.RS by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the DSL v1.
*/

/// Importing the "Serialize"
/// trait to derive it for enums.
use serde::Serialize;

/// Importing the "Deserialize"
/// trait to derive it for enums.
use serde::Deserialize;

/// An enum to
/// supply request types
/// in a Rustacean way.
/// Needed for the
/// `fetch_json` function
/// from "./network.rs".
#[derive(PartialEq)]
pub enum HTTPMethods {
    POST,
    GET
}

/// An enum to reflect
/// and store the online status
/// of a user. Needed for the
/// `SharkeyUser` structure
/// from "./responses.rs".
#[derive(Serialize, Deserialize)]
pub enum OnlineStatus{
    #[serde(rename(serialize = "unknown"))]
    #[serde(rename(deserialize = "unknown"))]
    Unknown,
    #[serde(rename(serialize = "online"))]
    #[serde(rename(deserialize = "online"))]
    Online,

    #[serde(rename(serialize = "active"))]
    #[serde(rename(deserialize = "active"))]
    Active,

    #[serde(rename(serialize = "offline"))]
    #[serde(rename(deserialize = "offline"))]
    Offline
}

/// An enum to reflect
/// and store the status
/// of reaction acceptance
/// for a note. Needed for the
/// `UserNote` function
/// from "./responses.rs".
#[derive(Serialize, Deserialize)]
pub enum ReactionAcceptance {
    #[serde(rename(serialize = "likeOnly"))]
    #[serde(rename(deserialize = "likeOnly"))]
    LikeOnly,

    #[serde(rename(serialize = "nonSensitiveOnly"))]
    #[serde(rename(deserialize = "nonSensitiveOnly"))]
    NonSensitiveOnly,

    #[serde(rename(serialize = "likeOnlyForRemote"))]
    #[serde(rename(deserialize = "likeOnlyForRemote"))]
    LikeOnlyForRemote,

    #[serde(rename(serialize = "nonSensitiveOnlyForLocalLikeOnlyForRemote"))]
    #[serde(rename(deserialize = "nonSensitiveOnlyForLocalLikeOnlyForRemote"))]
    NonSensitiveOnlyForLocalLikeOnlyForRemote
}

/// An enum to reflect
/// and store the status
/// of reaction acceptance
/// for a note. Needed for the
/// `UserNote`` function
/// from "./responses.rs" and
/// the `CreateNotePayload` from
/// "./payloads.rs".
#[derive(Serialize, Deserialize)]
pub enum NoteVisibility {

    #[serde(rename(serialize = "home"))]
    #[serde(rename(deserialize = "home"))]
    Home,

    #[serde(rename(serialize = "public"))]
    #[serde(rename(deserialize = "public"))]
    Public,

    #[serde(rename(serialize = "specified"))]
    #[serde(rename(deserialize = "specified"))]
    Specified,

    #[serde(rename(serialize = "followers"))]
    #[serde(rename(deserialize = "followers"))]
    Followers
}