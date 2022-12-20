use std::fmt::Error;

use env_logger::Env;
use lib::get_session;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Set INFO logging level as default
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    log::debug!("application started... try this!");

    let _ignore = lib::add(1, 2);
    println!("Hello, world!");

    let body = get_session(String::from("192.168.1.170")).await?;
    println!(body);

    Ok(())
}
