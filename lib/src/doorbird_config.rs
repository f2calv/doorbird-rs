
#[derive(Debug, serde::Deserialize)]
pub struct DoorbirdConfig {
    pub ip: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl std::fmt::Display for DoorbirdConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ip='{:?}', username='{:?}', username='*******'",
            self.ip, self.username
        )
    }
}
