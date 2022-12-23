use crate::{doorbird_config::DoorbirdConfig, doorbird_models::InfoResponse};
use reqwest::Error;
use serde::{Deserialize, Serialize};

pub struct Doorbird {
    pub ip: String,
    pub username: String,
    pub password: String,
}

impl Doorbird {
    pub fn new(config: DoorbirdConfig) -> Self {
        Self {
            ip: config.ip.unwrap_or_default(),
            username: config.username.unwrap_or_default(),
            password: config.password.unwrap_or_default(),
        }
    }

    pub async fn get_session(&self) -> Result<SessionResponse, Error> {
        let url = format!("http://{}/bha-api/getsession.cgi", self.ip);

        let client = reqwest::Client::new();

        let response = client
            .get(url)
            .basic_auth(&self.username, Some(&self.password))
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

    pub async fn invalidate_session(
        &self,
        old_session_id: String,
    ) -> Result<SessionResponse, Error> {
        let url = format!(
            "http://{}/bha-api/getsession.cgi?invalidate={}",
            self.ip, old_session_id
        );

        let client = reqwest::Client::new();

        let response = client
            .get(url)
            .basic_auth(&self.username, Some(&self.password))
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

    pub async fn get_live_image(&self) -> Result<bytes::Bytes, Error> {
        let url = format!("http://{}/bha-api/image.cgi", self.ip);

        let client = reqwest::Client::new();

        let bytes = client
            .get(url)
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await?
            .bytes()
            .await?;

        Ok(bytes)
    }

    pub async fn get_info(&self) -> Result<InfoResponse, Error> {
        let url = format!("http://{}/bha-api/info.cgi", self.ip);

        let client = reqwest::Client::new();

        let response = client
            .get(url)
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await?;
        //Note: lets get something we can log prior to deserialization
        //let res = response.json::<InfoResponse>().await?;

        let json = response.text().await?;
        //println!("{:?}", json);
        log::debug!("json={}", json);
        let res: Result<InfoResponse, serde_json::Error> = serde_json::from_str(json.as_str());
        let res = res.unwrap();

        Ok(res)
    }
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
