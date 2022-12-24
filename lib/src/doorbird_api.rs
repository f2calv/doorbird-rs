use crate::{doorbird_config::DoorbirdConfig, doorbird_models::*};
use reqwest::{Error, Url};

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

    pub async fn add_favorite(
        &self,
        _type: String,
        title: String,
        value: String,
        id: Option<String>,
    ) -> Result<String, Error> {
        let url = Url::parse_with_params(
            format!("http://{}/bha-api/favorites.cgi", self.ip).as_str(),
            &[
                ("action", "save"),
                ("type", _type.as_str()),
                ("title", title.as_str()),
                ("value", value.as_str()),
            ],
        )
        .unwrap();

        if id.is_some() {
            //let url = url.join(format!("&id={}", id.unwrap().as_str());
        }

        //let url = format!("http://{}/bha-api/favorites.cgi?action=save", self.ip);

        // let favourite = Favorite {
        //     _type,
        //     title,
        //     value,
        //     id: None,
        // };
        // let url = format!("{}&{}", url, favourite);
        //let url = "http://192.168.1.170/bha-api/favorites.cgi?action=save&type=http&title=RingServ&value=https://172.17.1.5/notify/ring";

        log::debug!("url={}", url);
        let client = reqwest::Client::new();

        let response = client
            .get(url)
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await?;
        //Note: lets get something we can log prior to deserialization
        //let res = response.json::<SessionResponse>().await?;

        //let json = &response.text().await?;

        let status = response.status().to_string();

        Ok(status)

        ////println!("{:?}", json);
        // log::debug!("json={}", json);
        // let res: Result<SessionResponse, serde_json::Error> = serde_json::from_str(json.as_str());
        // let res = res.unwrap();

        // Ok(res)
    }

    pub async fn open_door(
        &self,
        //door_controller_id: String,
        //relay: String,
    ) -> Result<String, Error> {
        let url = format!("http://{}/bha-api/open-door.cgi", self.ip);
        log::debug!("url={}", url);

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
