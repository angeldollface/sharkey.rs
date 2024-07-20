use tokio;
use chrono::prelude::Local;
use reqwest;
use std::env::var;
use std::io::Error;
use tokio::task::spawn;
use serde::Deserialize;
use serde_json::from_str;
use sharkey::NoteVisibility;
use sharkey::ReactionAcceptance;
use sharkey::create_note_for_user;

#[derive(Deserialize)]
struct UsersOnline {
    count: u32
}

impl UsersOnline{
    pub fn to_string(self) -> String {
        self.count.to_string()
    }
}

pub async fn get_count_and_post(
    url: &str,
    api_base: &str,
    base_url: &str,
    token: &String,
    reaction_acceptance: &ReactionAcceptance,
    visibility: &NoteVisibility
) -> Result<(), Error>{
    let resp = match reqwest::get(url).await {
        Ok(resp) => resp,
        Err(e) => return Err::<(), Error>(Error::new(std::io::ErrorKind::Other,e))
    };
    let body: String = match resp.text().await{
        Ok(body) => body,
        Err(e) => return Err::<(), Error>(Error::new(std::io::ErrorKind::Other,e))
    };
    let parsed: UsersOnline = match from_str(&body){
        Ok(parsed) => parsed,
        Err(e) => return Err::<(), Error>(Error::new(std::io::ErrorKind::Other,e))
    };
    let local: String = Local::now().to_string();
    let proj_url: String = "https://github.com/angeldollface/sharkey.rs/tree/main/example".to_string();
    let msg: String = format!(
        "Posted at: {}\nUsers online: {}\nPosted from: {}",
        local, parsed.to_string(), proj_url
    );
    let _posted = match create_note_for_user(
        api_base, 
        base_url, 
        &token, 
        visibility, 
        &Some(*reaction_acceptance), 
        &msg
    ).await {
        Ok(_posted) => _posted,
        Err(e) => return Err::<(), Error>(Error::new(std::io::ErrorKind::Other, e))
    };
    Ok(())
}

#[tokio::main]
async fn main() {
    match var("BLAHAJ_API_TOKEN"){
        Ok(token) => {
            let scheduler = spawn(
                async {
                    let api_token: String = token;

                    // Runs every 12 hours.
                    let mut interval = tokio::time::interval(std::time::Duration::from_secs(43200));
                    loop {
                        interval.tick().await;
                        match get_count_and_post(
                            "https://blahaj.zone/api/get-online-users-count", 
                            "/api", 
                            "https://blahaj.zone", 
                            &api_token, 
                            &ReactionAcceptance::LikeOnly, 
                            &NoteVisibility::Public
                        ).await {
                            Ok(_x) => {},
                            Err(e) => eprintln!("{}", e)
                        };
                    }
                }
            );
            let _ = scheduler.await;
        },
        Err(e) => eprintln!("{}", e)
    }    
}
