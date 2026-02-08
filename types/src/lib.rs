#![no_std]

pub mod constant;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub enum MessagePcToBoard {
    Ask,
}

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub enum MessageBoardToPc {
    State(bool),
}
