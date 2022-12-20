use reqwest::Error;
use serde::{Deserialize, Serialize};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
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

pub async fn get_session(device_ip: String) -> Result<String, Error> {
    let url = format!("http://{device_ip}/bha-api/getsession.cgi");

    let body = reqwest::get(url).await?.text().await?;

    Ok(body)
}
