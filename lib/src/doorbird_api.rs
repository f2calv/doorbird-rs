use reqwest::Error;
use serde::{Deserialize, Serialize};

use crate::doorbird_config::DoorbirdConfig;

pub async fn get_session(doorbird_config: DoorbirdConfig) -> Result<String, Error> {
    let url = format!(
        "http://{}/bha-api/getsession.cgi",
        doorbird_config.ip.unwrap()
    );

    //let response = reqwest::get(&url).await?;

    let client = reqwest::Client::new();

    let response = client
        .get(url)
        .basic_auth(doorbird_config.username.unwrap(), doorbird_config.password)
        .send()
        .await?;

    //let json = response.json().await?;

    let body = response.text().await?;
    //println!("{:?}", body);

    Ok(body)
}

pub struct SessionResponse {
    pub bha: BHASession,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BHASession {
    pub return_code: String,
    pub session_id: String,
    pub encryption_type: i32,
    pub encryption_key: String,
}

impl std::fmt::Display for BHASession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RETURNCODE='{:?}', SESSIONID='{:?}', ENCRYPTION_TYPE='{:?}', ENCRYPTION_KEY='{:?}'",
            self.return_code, self.session_id, self.encryption_type, self.encryption_key
        )
    }
}
