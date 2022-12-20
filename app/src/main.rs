use env_logger::Env;
use error_stack::{IntoReport, Result, ResultExt};
use lib::get_session;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Set INFO logging level as default
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    //Load app settings from env variables
    let _app_settings = configuration::get_configuration().expect("configuration issue");

    log::debug!("application started... try this!");

    let body = get_session(
        _app_settings.doorbird_config.ip.unwrap(),
        _app_settings.doorbird_config.username.unwrap(),
        _app_settings.doorbird_config.password,
    )
    .await
    .into_report()
    .change_context(std::io::Error::new(
        std::io::ErrorKind::Other,
        "some API error happened here",
    ))
    .attach_printable("somthing failed")?;
    println!("{}", body);

    Ok(())
}

mod configuration {

    use config::{Config, ConfigError, File};

    pub fn get_configuration() -> Result<AppSettings, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name("appsettings.toml"))
            .add_source(
                config::Environment::with_prefix("APP")
                    .prefix_separator("_")
                    .separator("__"),
            )
            .build()?;
        config.try_deserialize()
    }

    #[derive(Debug, serde::Deserialize)]
    pub struct AppSettings {
        pub application: AppConfig,
        pub doorbird_config: DoorbirdConfig,
    }

    impl std::fmt::Display for AppSettings {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "application='{:?}', doorbird_config='{:?}'",
                self.application, self.doorbird_config
            )
        }
    }

    #[derive(Debug, serde::Deserialize)]
    pub struct AppConfig {
        pub host: Option<String>,
        pub port: Option<u16>,
    }

    impl std::fmt::Display for AppConfig {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "host='{:?}', port='{:?}'", self.host, self.port)
        }
    }

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
}
