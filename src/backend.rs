use std::{env, time::SystemTime};

use log::debug;
use reqwest::{self, Error};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub(crate) struct Message {
    pub(crate) id: u64,
    pub(crate) content: String,
    pub(crate) timestamp: SystemTime,
    pub(crate) user_id: u64,
    pub(crate) user_name: String,
    pub(crate) guild_id: u64,
    pub(crate) guild_name: String,
    pub(crate) channel_id: u64,
    pub(crate) channel_name: String,
    pub(crate) edited_timestamp: Option<SystemTime>,
}

pub(crate) async fn update_message(msg: Message) -> Result<(), Error> {
    let base_url = env::var("BACKEND_API").expect("Missing Env var: BACKEND_API");
    let client = reqwest::Client::new();

    debug!("POST {base_url}/message | {msg:?}");
    let r = client
        .post(format!("{base_url}/message"))
        .json(&msg)
        .send()
        .await?;
    if r.status() == reqwest::StatusCode::OK {
        Ok(())
    } else {
        let e = r.error_for_status().err().unwrap();
        Err(e)
    }
}
