extern crate multiinput;

use multiinput::*;
use anyhow::Result;
use serde::Deserialize;
use obws::Client;
use std::fs;
use toml;

mod router;
mod processes;

#[derive(Debug, Deserialize)]
struct Config {
    obs_client: ObsClientConfig
}

#[derive(Debug, Deserialize)]
struct ObsClientConfig {
    host: Option<String>,
    port: Option<u16>,
    password: Option<String>
}

async fn init_client_from_config(path: &str) -> Result<Client> {
    let s = fs::read_to_string(path)?;
    let config: Config = toml::from_str(s.as_str())?;
    match config.obs_client {
        ObsClientConfig { host: Some(host), port: Some(port), password } => {
            Result::Ok(Client::connect(host, port, password).await?)
        }

        _ => {
            Result::Err(anyhow::anyhow!("Invalid config."))
        }
    }
}


fn init_router() -> router::Router {
    let router  = router::Router::init(
        processes::init_processes()
    );

    router
}

#[tokio::main]
async fn main() -> Result<()> {

    let client = init_client_from_config("./config.toml").await?;
    let router = init_router();
    router.process(&client, 1).await?;

    let mut manager = RawInputManager::new().unwrap();
    manager.register_devices(DeviceType::Joysticks(XInputInclude::True));
    manager.register_devices(DeviceType::Keyboards);
    manager.register_devices(DeviceType::Mice);
    'outer: loop{
        if let Some(event) = manager.get_event(){
            match event{
                RawEvent::KeyboardEvent(_,  KeyId::Escape, State::Pressed)
                    => break 'outer,
                _ => (),
            }
            println!("{:?}", event);
        }
    }
    println!("Finishing");
    Ok(())
}