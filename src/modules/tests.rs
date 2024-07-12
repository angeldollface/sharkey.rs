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
            match resp.body {
                Some(val) => {
                    assert_eq!(val.is_empty(), false);
                },
                None => {
                    println!("Could not fetch response!");
                }
            }
        },
        Err(e) => {
            println!("{}", e);
        }
    }
}

/// The function to test
/// the "like_note_for_user" and the
/// "unlike_note_for_user" function.
#[tokio::test]
pub async fn test_note_reaction(){
    match std::env::var("BLAHAJ_API_TOKEN"){
        Ok(value) => {
            let like_payload: super::payloads::ReactionPayload = super::payloads::ReactionPayload{
                note_id: "9utzyrsmyoof00hr".to_string(),
                reaction: "like".to_string(),
                i: value.clone()
            };
            let unlike_payload: super::payloads::ReactionPayload = super::payloads::ReactionPayload{
                note_id: "9utzyrsmyoof00hr".to_string(),
                reaction: "like".to_string(),
                i: value.clone()
            };
            match super::actions::like_note_for_user(
                "/api", 
                "https://blahaj.zone", 
                &like_payload
            ).await {
                Ok(res) => {
                    match res.body {
                        Some(val) => {
                            println!("{}", val);
                        },
                        None => {
                            assert_eq!(false, false);
                            match super::actions::unlike_note_for_user(
                                "/api", 
                                "https://blahaj.zone", 
                                &unlike_payload
                            ).await {
                                Ok(res) => {
                                    match res.body {
                                        Some(val) => println!("{}", val),
                                        None => {
                                            assert_eq!(false, false);
                                        }
                                    }
                                },
                                Err(x) => {
                                    println!("{}", x);
                                }
                            }
                        }
                    }
                },
                Err(x) => {
                    println!("{}", x);
                }
            }
        },
        Err(e) => {
            println!("{}", e);
        }
    }    
}

/// The function to test
/// the "follow_user" and the 
/// "unfollow_user" function.
#[tokio::test]
pub async fn test_follow_action_user(){
    match std::env::var("BLAHAJ_API_TOKEN"){
        Ok(value) => {
            let unfollow_payload: super::payloads::UnfollowPayload = super::payloads::UnfollowPayload{
                i: value.clone(),
                user_id: "9upmnr8igmxe01k3".to_string()
            };
            let follow_payload: super::payloads::FollowPayload = super::payloads::FollowPayload{
                user_id: "9upmnr8igmxe01k3".to_string(),
                with_replies: false,
                i: value.clone()
            };
            match super::actions::follow_user(
                "/api", 
                "https://blahaj.zone", 
                &follow_payload
            ).await {
                Ok(res) => {
                    assert_eq!(res.username, "frisaf");
                    match super::actions::unfollow_user(
                        "/api", 
                        "https://blahaj.zone", 
                        &unfollow_payload
                    ).await {
                        Ok(res) => {
                            assert_eq!(res.username, "frisaf");
                        },
                        Err(x) => {
                            println!("{}", x);
                        }
                    }
                },
                Err(y) => {
                    println!("{}", y);
                }
            }
        },
        Err(e) => {
            println!("{}", e);
        }
    }    
}

/// The function to test
/// the "create_note_for_user" function.
#[tokio::test]
pub async fn test_create_note_for_user(){
    match std::env::var("BLAHAJ_API_TOKEN"){
        Ok(value) => {
            let payload: super::payloads::CreateNotePayload = super::payloads::CreateNotePayload{
                visibility: super::enums::NoteVisibility::Public,
                cw: None,
                local_only: true,
                reaction_acceptance: Some(super::enums::ReactionAcceptance::LikeOnly),
                no_extract_mentions: false,
                no_extract_hashtags: false,
                no_extract_emojis: false,
                reply_id: None,
                channel_id: None,
                text: "This note was posted from the test runner of \"Sharkey.rs\"!".to_string(),
                i: value
            };
            match super::actions::create_note_for_user(
                "/api", 
                "https://blahaj.zone", 
                &payload
            ).await {
                Ok(res) => {
                    assert_eq!(&res.created_note.user.username, &"angeldollface666".to_string());
                },
                Err(x) => {
                    println!("{}", x);
                }
            }
        },
        Err(e) => {
            println!("{}", e);
        }
    }    
}

/// The function to test
/// the "delete_note_for_user" function.
#[tokio::test]
pub async fn test_delete_note_for_user(){
    match std::env::var("BLAHAJ_API_TOKEN"){
        Ok(value) => {
            let payload: super::payloads::CreateNotePayload = super::payloads::CreateNotePayload{
                visibility: super::enums::NoteVisibility::Public,
                cw: None,
                local_only: true,
                reaction_acceptance: Some(super::enums::ReactionAcceptance::LikeOnly),
                no_extract_mentions: false,
                no_extract_hashtags: false,
                no_extract_emojis: false,
                reply_id: None,
                channel_id: None,
                text: "This note only exists to be deleted.".to_string(),
                i: value.clone()
            };
            match super::actions::create_note_for_user(
                "/api", 
                "https://blahaj.zone", 
                &payload
            ).await {
                Ok(res) => {
                    let note_to_be_deleted = &res.created_note.id;
                    let del_payload = &super::payloads::DeleteNotePayload{
                        note_id: note_to_be_deleted.to_owned(),
                        i: value.clone()
                    };
                    match super::actions::delete_note_for_user(
                        "/api", 
                        "https://blahaj.zone", 
                        del_payload
                    ).await {
                        Ok(resp) => {
                            assert_eq!(resp.body, None);
                        },
                        Err(y) => println!("{}", y)
                    };
                    //assert_eq!(&res.created_note.user.username, &"angeldollface666".to_string());
                },
                Err(x) => {
                    println!("{}", x);
                }
            }
        },
        Err(e) => {
            println!("{}", e);
        }
    }    
}