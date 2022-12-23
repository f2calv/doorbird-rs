use crate::{doorbird_config::DoorbirdConfig, doorbird_models::*};
use reqwest::Error;

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

    pub async fn get_favorites(&self) -> Result<String, Error> {
        let url = format!("http://{}/bha-api/favorites.cgi", self.ip);

        let client = reqwest::Client::new();

        let response = client
            .get(url)
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await?;
        //Note: lets get something we can log prior to deserialization
        //let res = response.json::<SessionResponse>().await?;

        let json = response.text().await?;
        Ok(json)

        //TODO: create favorite object

        ////println!("{:?}", json);
        // log::debug!("json={}", json);
        // let res: Result<SessionResponse, serde_json::Error> = serde_json::from_str(json.as_str());
        // let res = res.unwrap();

        // Ok(res)
    }
}
