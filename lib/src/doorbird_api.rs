use reqwest::Error;
use serde::{Deserialize, Serialize};

use crate::doorbird_config::DoorbirdConfig;

pub async fn get_session(doorbird_config: DoorbirdConfig) -> Result<SessionResponse, Error> {
    let url = format!(
        "http://{}/bha-api/getsession.cgi",
        doorbird_config.ip.unwrap()
    );

    let client = reqwest::Client::new();

    let response = client
        .get(url)
        .basic_auth(doorbird_config.username.unwrap(), doorbird_config.password)
        .send()
        .await?;
    //Note: lets get something we can log prior to deserialization
    //let res = response.json::<SessionResponse>().await?;

    let json = response.text().await?;
    //println!("{:?}", json);
    log::debug!("json={}", json);
    let res: Result<SessionResponse, serde_json::Error> = serde_json::from_str(json.as_str());
    let res = res.unwrap();

    Ok(res)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionResponse {
    #[serde(rename = "BHA")]
    pub bha: BHASession,
}

impl std::fmt::Display for SessionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BHA='{:?}'", self.bha)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BHASession {
    #[serde(rename = "RETURNCODE")]
    pub return_code: String,
    #[serde(rename = "SESSIONID")]
    pub session_id: String,
    #[serde(rename = "ENCRYPTION_TYPE")]
    pub encryption_type: i32,
    #[serde(rename = "ENCRYPTION_KEY")]
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
