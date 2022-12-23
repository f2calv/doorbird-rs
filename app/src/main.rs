use env_logger::Env;
use error_stack::Result;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Set INFO logging level as default
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    //Load app settings from env variables
    let _app_settings = configuration::get_configuration().expect("configuration issue");

    log::debug!("application started...");

    let doorbird = lib::doorbird_api::Doorbird::new(_app_settings.doorbird_config);

    let res = doorbird.get_session().await.unwrap();
    // .into_report()
    // .change_context(std::io::Error::new(
    //     std::io::ErrorKind::Other,
    //     "some API error happened here",
    // ))
    // .attach_printable("somthing failed")?;
    println!("json={}", res);

    let bytes = doorbird.get_live_image().await.unwrap();
    println!("bytes={}", bytes.len());

    let info = doorbird.get_info().await.unwrap();
    println!("info={}", info);

    let favorites = doorbird.get_favorites().await.unwrap();
    println!("favorites={}", favorites);

    // doorbird
    //     .add_favorite(
    //         String::from("http"),
    //         String::from("test title"),
    //         String::from("https://www.google.com"),
    //     )
    //     .await
    //     .unwrap();
    Ok(())
}

mod configuration {

    use config::{Config, ConfigError, File};

    pub fn get_configuration() -> Result<AppSettings, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name("appsettings.toml"))
            .add_source(File::with_name("appsettings.local.toml").required(false))
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
        pub doorbird_config: lib::doorbird_config::DoorbirdConfig,
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
}
