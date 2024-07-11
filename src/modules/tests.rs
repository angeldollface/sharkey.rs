/*
SHARKEY.RS by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the DSL v1.
*/

/// Importing the
/// "tokio" namespace
/// to use the "test"
/// macro.
use tokio;

/// The function to test
/// the "fetch_json" function.
#[tokio::test]
pub async fn test_fetch_json(){
    let mut payload: std::collections::HashMap<String,String> = std::collections::HashMap::new();
    payload.insert("lib_name".to_string(), "sharkey.rs".to_string());
    match super::network::fetch_json(
        &super::enums::HTTPMethods::GET, 
        &payload, 
        "https://httpbin.org/json"
    ).await {
        Ok(resp) => {
            assert_eq!(resp.status(), 200);
        },
        Err(e) => {
            println!("{}", e);
        }
    }
}