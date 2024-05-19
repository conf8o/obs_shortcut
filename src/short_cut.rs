//! 具体的な動作の定義
//! init_processesにusize(= usize)と動作のペアを設定する。(TODO: 書き方もう少し簡素化したい。)
//! eventsには、デバイスの入力 → usizeを割り出す定義を行う。

use obws::{Client, Result};
use crate::router::Process;

pub const A: usize = 0;
pub const B: usize = 1;
pub const C: usize = 2;
pub const D: usize = 3;
pub const E: usize = 4;
pub const F: usize = 5;
pub const G: usize = 6;
pub const H: usize = 7;
pub const I: usize = 8;
pub const J: usize = 9;
pub const K: usize = 10;
pub const L: usize = 11;
pub const M: usize = 12;
pub const N: usize = 13;
pub const O: usize = 14;
pub const P: usize = 15;
pub const Q: usize = 16;
pub const R: usize = 17;
pub const S: usize = 18;
pub const T: usize = 19;
pub const U: usize = 20;
pub const V: usize = 21;
pub const W: usize = 22;
pub const X: usize = 23;
pub const Y: usize = 24;
pub const Z: usize = 25;

pub fn init_shortcuts() -> Vec<(usize, Process)> {
    vec![
        (A, |client: &Client| Box::pin(set_current_program_scene(client, "ANGRY BREAK PC"))),
        (B, |client: &Client| Box::pin(set_current_program_scene(client, "Boy's Staredown"))),
        (D, |client: &Client| Box::pin(set_current_program_scene(client, "DIRECTED BY ROBERT"))),
        (E, |client: &Client| Box::pin(set_current_program_scene(client, "Explosion"))),
        (F, |client: &Client| Box::pin(set_current_program_scene(client, "FACEPALM"))),
        (P, |client: &Client| Box::pin(set_current_program_scene(client, "メイン"))),
        (S, |client: &Client| Box::pin(set_current_program_scene(client, "SPACE CAT"))),
        (W, |client: &Client| Box::pin(set_current_program_scene(client, "WHAT HOW"))),
        (R, |client: &Client| Box::pin(set_current_program_scene(client, "R WHO ARE U")))
    ]
}

async fn set_current_program_scene(client: &Client, name: &str) -> Result<()> {
    client.scenes().set_current_program_scene(name).await
}