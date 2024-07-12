/*
SHARKEY.RS by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the DSL v1.
*/

/// Importing the "Bridge"
/// structure to handle
/// all kinds of responses.
use super::bridge::Bridge;

/// Importing the "SharkeyErr"
/// structure for handling errors.
use super::error::SharkeyErr;

/// Importing the "HTTPMethods"
/// enum to make POST requests.
use super::enums::HTTPMethods;

/// Importing the "fetch_json"
/// to make network requests.
use super::network::fetch_json;

/// Importing the "SharkeyUser"
/// structure.
use super::responses::SharkeyUser;

/// Importing the "CreatedNote"
/// structure.
use super::responses::CreatedNote;

/// Importing the "FollowPayload"
/// structure.
use super::payloads::FollowPayload;

/// Importing the "ReactionPayload"
/// structure.
use super::payloads::ReactionPayload;

/// Importing the "UnfollowPayload"
/// structure.
use super::payloads::UnfollowPayload;

/// Importing the "CreateNotePayload"
/// structure.
use super::payloads::CreateNotePayload;

/// Importing the "DeleteNotePayload"
/// structure.
use super::payloads::DeleteNotePayload;

/// Attempts to delete a note for a user.
/// If successful, an instance of the 
/// `OperationStatus` structure is returned.
/// If not, an error is returned.
pub async fn delete_note_for_user(
    api_base: &str,
    base_url: &str,
    payload: &DeleteNotePayload
) -> Result<Bridge, SharkeyErr> {
    let url: String = format!("{}{}/notes/delete", base_url, api_base);
    let result: Bridge = match fetch_json(
        &HTTPMethods::POST, 
        payload, &url
    ).await {
        Ok(response) => response,
        Err(e) => return Err::<Bridge, SharkeyErr>(SharkeyErr::new(&e.to_string()))
    };
    Ok(result)
}

/// Attempts to create a note for a user.
/// If successful, an instance of the 
/// `CreatedNote` structure is returned.
/// If not, an error is returned.
pub async fn create_note_for_user(
    api_base: &str,
    base_url: &str,
    payload: &CreateNotePayload
) -> Result<CreatedNote, SharkeyErr> {
    let url: String = format!("{}{}/notes/create", base_url, api_base);
    let response: Bridge = match fetch_json(
        &HTTPMethods::POST, 
        payload, &url
    ).await {
        Ok(response) => response,
        Err(e) => return Err::<CreatedNote, SharkeyErr>(SharkeyErr::new(&e.to_string()))
    };
    let body: String = match response.body {
        Some(body) => body,
        None => {
            let e: String = "No valid response received!".to_string();
            return Err::<CreatedNote, SharkeyErr>(SharkeyErr::new(&e.to_string()))
        }
    };
    let result: CreatedNote = match serde_json::from_str(&body){
        Ok(result) => result,
        Err(e) => return Err::<CreatedNote, SharkeyErr>(SharkeyErr::new(&e.to_string()))
    };
    Ok(result)
}

/// Attempts to react to a note for a user.
/// If successful, an instance of the 
/// `OperationStatus` structure is returned.
/// If not, an error is returned.
pub async fn like_note_for_user(
    api_base: &str,
    base_url: &str,
    payload: &ReactionPayload
) -> Result<Bridge, SharkeyErr> {
    let url: String = format!("{}{}/notes/reactions/create", base_url, api_base);
    let result: Bridge = match fetch_json(
        &HTTPMethods::POST, 
        payload, &url
    ).await {
        Ok(response) => response,
        Err(e) => return Err::<Bridge, SharkeyErr>(SharkeyErr::new(&e.to_string()))
    };
    Ok(result)
    
}

/// Attempts to reverse a reaction to a note for a user.
/// If successful, an instance of the 
/// `OperationStatus` structure is returned.
/// If not, an error is returned.
pub async fn unlike_note_for_user(
    api_base: &str,
    base_url: &str,
    payload: &ReactionPayload
) -> Result<Bridge, SharkeyErr> {
    let url: String = format!("{}{}/notes/reactions/delete", base_url, api_base);
    let result: Bridge = match fetch_json(
        &HTTPMethods::POST, 
        payload, 
        &url
    ).await {
        Ok(response) => response,
        Err(e) => return Err::<Bridge, SharkeyErr>(SharkeyErr::new(&e.to_string()))
    };
    Ok(result)
    
}

/// Attempts to follow a user.
/// If successful, an instance of the 
/// `SharkeyUser` structure is returned.
/// If not, an error is returned.
pub async fn follow_user(
    api_base: &str,
    base_url: &str,
    payload: &FollowPayload
) -> Result<SharkeyUser, SharkeyErr> {
    let url: String = format!("{}{}/following/create", base_url, api_base);
    let response: Bridge = match fetch_json(
        &HTTPMethods::POST, 
        payload, &url
    ).await {
        Ok(response) => response,
        Err(e) => return Err::<SharkeyUser, SharkeyErr>(SharkeyErr::new(&e.to_string()))
    };
    let body: String = match response.body {
        Some(body) => body,
        None => {
            let e: String = "No valid response received!".to_string();
            return Err::<SharkeyUser, SharkeyErr>(SharkeyErr::new(&e.to_string()))
        }
    };
    let result: SharkeyUser = match serde_json::from_str(&body) {
        Ok(result) => result,
        Err(e) => return Err::<SharkeyUser, SharkeyErr>(SharkeyErr::new(&e.to_string()))
    };
    Ok(result)
}

/// Attempts to unfollow a user.
/// If successful, an instance of the 
/// `SharkeyUser` structure is returned.
/// If not, an error is returned.
pub async fn unfollow_user(
    api_base: &str,
    base_url: &str,
    payload: &UnfollowPayload
) -> Result<SharkeyUser, SharkeyErr> {
    let url: String = format!("{}{}/following/delete", base_url, api_base); 
    let response: Bridge = match fetch_json(
        &HTTPMethods::POST, 
        payload, &url
    ).await {
        Ok(response) => response,
        Err(e) => return Err::<SharkeyUser, SharkeyErr>(SharkeyErr::new(&e.to_string()))
    };  
    let body: String = match response.body {
        Some(body) => body,
        None => {
            let e: String = "No valid response received!".to_string();
            return Err::<SharkeyUser, SharkeyErr>(SharkeyErr::new(&e.to_string()))
        }
    };
    let result: SharkeyUser = match serde_json::from_str(&body) {
        Ok(result) => result,
        Err(e) => return Err::<SharkeyUser, SharkeyErr>(SharkeyErr::new(&e.to_string()))
    };
    Ok(result)
}