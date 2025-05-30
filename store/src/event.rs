use std::collections::HashMap;

use crate::*;
use serde::{Deserialize, Serialize};
/// An event that progresses the GameState forward
#[derive(Debug, Clone, Serialize, PartialEq, Deserialize)]
pub enum GameEvent {
    BeginGame {
        player_list: HashMap<u8, Players>,
    },
    EndGame,
    AccessForbidden,
    Impact {
        id: u8,
    },
    Death {
        player_id: u8,
    },
    PlayerJoined {
        player_id: u8,
        name: String,
        position: Position,
        client_id: u64,
    },
    PlayerDisconnected {
        player_id: u8,
    },
    PlayerMove {
        player_id: u8,
        at: Position,
        player_list: HashMap<u8, Players>,
        vision: (f32, f32),
    },
    Spawn {
        player_id: u8,
        position: Position,
        lvl: usize,
    },
    Timer {
        duration: u8,
    },
}
