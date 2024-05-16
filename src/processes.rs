//! 具体的な動作の定義
//! init_processesにProcessIndex(= usize)と動作のペアを設定する。(TODO: 書き方もう少し簡素化したい。)
//! eventsには、デバイスの入力 → ProcessIndexを割り出す定義を行う。

use obws::{Client, Result};
use crate::router::{ ProcessIndex, Process };

pub const W: ProcessIndex = 0;
pub const B: ProcessIndex = 1;

pub fn init_processes() -> Vec<(ProcessIndex, Process)> {
    vec![
        (W, |client: &Client| Box::pin(set_current_program_scene(client, "WHAT HOW"))),
        (B, |client: &Client| Box::pin(set_current_program_scene(client, "Boy's Staredown")))
    ]
}

async fn set_current_program_scene(client: &Client, name: &str) -> Result<()> {
    client.scenes().set_current_program_scene(name).await
}