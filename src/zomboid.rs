use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Instant};

#[derive(Clone, Debug, Deserialize)]
pub struct PlayerDTO {
    x: f32,
    y: f32,
    username: String,
    forename: String,
    surname: String,
}

#[derive(Clone, Debug)]
pub struct State {
    players: HashMap<String, Player>,
}

impl State {
    pub fn new() -> Self {
        let players = HashMap::new();
        Self { players }
    }

    pub fn from(state: &State) -> Self {
        Self {
            players: state.players.clone(),
        }
    }

    pub fn players(&self) -> &HashMap<String, Player> {
        &self.players
    }

    pub fn update_player(&mut self, player: &PlayerDTO) {
        let name = player.username.clone();
        let player = Player::from_dto(player);
        self.players.insert(name, player);
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Position {
    x: f32,
    y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Player {
    character_name: String,
    name: String,
    position: Position,
    #[serde(skip_serializing)]
    last_updated_at: Instant,
}

impl Player {
    pub fn from_dto(player: &PlayerDTO) -> Self {
        let last_updated_at = Instant::now();
        let position = Position::new(player.x, player.y);
        let name = format!("{} {}", player.forename, player.surname);
        Self {
            position,
            name: player.username.clone(),
            character_name: name,
            last_updated_at,
        }
    }
}
