use env_logger::Env;
use error_stack::{IntoReport, Result, ResultExt};
use lib::get_session;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Set INFO logging level as default
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    log::debug!("application started... try this!");

    let body = get_session(String::from("192.168.1.170"))
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
