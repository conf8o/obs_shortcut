use obws::{Client, Result};
use crate::router::{ ProcessIndex, Process };

const PRESS_W: ProcessIndex = 0;
const PRESS_B: ProcessIndex = 1;

pub fn init_processes() -> Vec<(ProcessIndex, Process)> {
    vec![
        (PRESS_W, |client: &Client| Box::pin(set_current_program_scene(client, "WHAT HOW"))),
        (PRESS_B, |client: &Client| Box::pin(set_current_program_scene(client, "Boy's Staredown")))
    ]
}

async fn set_current_program_scene(client: &Client, name: &str) -> Result<()> {
    client.scenes().set_current_program_scene(name).await
}