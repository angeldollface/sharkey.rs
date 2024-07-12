/*
SHARKEY.RS by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the DSL v1.
*/

/// Importing the "UserNote"
/// structure.
use super::aux::UserNote;

/// Importing the "Bridge"
/// structure.
use super::responses::Bridge;

/// Importing the "SharkeyErr"
/// structure for handling errors.
use super::error::SharkeyErr;

/// Importing the "HTTPMethods"
/// enum to make POST requests.
use super::enums::HTTPMethods;

/// Importing the "HashMap"
/// structure from the standard
/// library to submit a simple
/// payload.
use std::collections::HashMap;

/// Importing the "fetch_json"
/// function to make network 
/// requests.
use super::network::fetch_json;

/// Importing the "SharkeyUser"
/// structure.
use super::responses::SharkeyUser;

/// Importing the "UserInfoPayload"
/// structure to attain user information.
use super::payloads::UserInfoPayload;

/// Importing the "UserNotesPayload"
/// structure.
use super::payloads::UserNotesPayload;

/// Attempts to fetch information on a user.
/// Returns an instance of the `SharkeyUser` structure.
/// If this fails, an error is returned.
pub async fn get_user_info(
    api_base: &str,
    base_url: &str,
    host: &str,
    username: &str
) -> Result<SharkeyUser, SharkeyErr> {
    let url: String = format!("{}{}/users/show", base_url, api_base);
    let payload: UserInfoPayload = UserInfoPayload{
        host: host.to_string(),
        detailed: false,
        username: username.to_string()
    };
    let response: Bridge = match fetch_json(
        &HTTPMethods::POST, 
        &payload, 
        &url
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

/// Attempts to retrieve user information 
/// from the API token submitted. Returns
/// an instance of the `SharkeyUser` structure.
/// If the operation fails, an error is returned.
pub async fn get_user_from_token(
    api_base: &str,
    base_url: &str,
    api_token: &str
) -> Result<SharkeyUser, SharkeyErr> {
    let url: String = format!("{}{}/i", base_url, api_base);
    let mut payload: HashMap<String, String> = HashMap::new();
    payload.insert("i".to_string(), api_token.to_string());
    let response: Bridge = match fetch_json(
        &HTTPMethods::POST, 
        &payload, 
        &url
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

/// Attempts to retrieve user notes
/// from the user ID submitted. Returns
/// a vector of instances of the `UserNote`
/// structure. If the operation fails, an 
/// error is returned.
pub async fn get_user_notes(
    api_base: &str,
    base_url: &str,
    user_id: &str
) -> Result<Vec<UserNote>, SharkeyErr> {
    let url: String = format!("{}{}/users/notes", base_url, api_base);
    let payload: UserNotesPayload = UserNotesPayload{
        user_id: user_id.to_string(),
        with_replies: false,
        with_renotes: false,
        with_files: false,
        allow_partial: true
    };
    let response: Bridge = match fetch_json(
        &HTTPMethods::POST, 
        &payload, 
        &url
    ).await {
        Ok(response) => response,
        Err(e) => return Err::<Vec<UserNote>, SharkeyErr>(SharkeyErr::new(&e.to_string()))
    };
    let body: String = match response.body {
        Some(body) => body,
        None => {
            let e: String = "No valid response received!".to_string();
            return Err::<Vec<UserNote>, SharkeyErr>(SharkeyErr::new(&e.to_string()))
        }
    };
    //println!("{}", &body);
    let result: Vec<UserNote> = match serde_json::from_str(&body) {
        Ok(result) => result,
        Err(e) => return Err::<Vec<UserNote>, SharkeyErr>(SharkeyErr::new(&e.to_string()))
    };
    Ok(result)
}