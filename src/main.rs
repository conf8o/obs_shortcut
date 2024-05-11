extern crate multiinput;

use multiinput::*;
use anyhow::Result;
mod router;
use serde::Deserialize;
use obws::Client;
use std::fs;
use toml;

#[derive(Debug, Deserialize)]
pub struct ObsClientConfig {
    host: Option<String>,
    port: Option<u16>,
    password: Option<String>
}

pub async fn init_client_from_config(path: &str) -> Result<Client> {
    let s = fs::read_to_string(path)?;
    let config: ObsClientConfig = toml::from_str(s.as_str())?;
    match config {
        ObsClientConfig { host: Some(host), port: Some(port), password } => {
            Result::Ok(Client::connect(host, port, password).await?)
        }

        _ => {
            Result::Err(anyhow::anyhow!("Invalid config."))
        }
            
    }
}

const PRESS_W: usize = 1;

fn init_router(obs_client: &Client) -> router::Router {
    let router  = router::Router::init(
        vec![
            (PRESS_W, router::no_process)
        ]
    );

    router
}

#[tokio::main]
async fn main() -> Result<()> {

    let client = init_client_from_config("./obs_connetion.toml").await?;

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