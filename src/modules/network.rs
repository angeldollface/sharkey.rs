/*
SHARKEY.RS by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the DSL v1.
*/

/// Importing the 
/// "Client" structure
/// to make a new client.
use reqwest::Client;

/// Importing the 
/// "Serialize" trait
/// to only accept structures
/// that implement this trait
/// as a JSON payload.
use serde::Serialize;

/// Importing the 
/// "Response" structure
/// to get back responses
/// from any type of request.
use reqwest::Response;

/// Importing the "Bridge"
/// structure to catch
/// responses of all kinds.
use super::bridge::Bridge;

/// Importing the 
/// "SharkeyErr" structure
/// to handle errors.
use super::error::SharkeyErr;

/// Importing the 
/// "HTTPMethods" enum
/// to submit the correct
/// request type.
use super::enums::HTTPMethods;

/// Importing the 
/// "CONTENT_TYPE" setting
/// to set the content type
/// of the response to JSON.
use reqwest::header::CONTENT_TYPE;

/// Attempts to fetch an instance
/// of the "Response" structure from
/// the submitted type of request with
/// the supplied request payload.
pub async fn fetch_json<T: Serialize>(
    method: &HTTPMethods,
    payload: &T,
    url: &str
) -> Result<Bridge, SharkeyErr> {
    let result: Bridge;
    let client = Client::new();
    if method == &HTTPMethods::GET {
        let resp: Response = match client.get(url)
            .header(CONTENT_TYPE, "application/json")
            .json(payload)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => return Err::<Bridge, SharkeyErr>(SharkeyErr::new(&e.to_string()))
        };
        let body: String = match resp.text().await {
            Ok(body) => body,
            Err(e) => return Err::<Bridge, SharkeyErr>(SharkeyErr::new(&e.to_string()))            
        };
        if body.clone().is_empty(){
            result = Bridge{ body: None };
        }
        else {
            result = Bridge{ body: Some(body.clone()) };
        }
    }
    else {
        let resp: Response = match client.post(url)
            .header(CONTENT_TYPE, "application/json")
            .json(payload)
            .send()
            .await
        {
            Ok(result) => result,
            Err(e) => return Err::<Bridge, SharkeyErr>(SharkeyErr::new(&e.to_string()))
        };
        let body: String = match resp.text().await {
            Ok(body) => body,
            Err(e) => return Err::<Bridge, SharkeyErr>(SharkeyErr::new(&e.to_string()))            
        };
        if body.clone().is_empty(){
            result = Bridge{ body: None };
        }
        else {
            result = Bridge{ body: Some(body.clone()) };
        }
    }
    Ok(result)
}