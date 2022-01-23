use crate::{Config, ZomboidWebMapError};
use rusqlite::Connection;
use serde::Serialize;
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

#[derive(Clone)]
pub struct State {
    world: World,
    players: Vec<Player>,
}

impl State {
    pub fn new(config: &Config) -> Self {
        let world = World::new(config);
        let players = world.load_players().unwrap();
        Self { world, players }
    }

    pub fn players(&self) -> &Vec<Player> {
        &self.players
    }
}

#[derive(Clone)]
pub struct World {
    name: String,
    root: PathBuf,
}

impl World {
    pub fn new(config: &Config) -> Self {
        let name = config.world_name.clone();
        let relative = PathBuf::from("Saves/Multiplayer").join(&name);
        let root = config.server_directory.join(&relative);
        World { root, name }
    }

    pub fn load_players(&self) -> Result<Vec<Player>, ZomboidWebMapError> {
        Player::load(&self)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Position {
    x: f32,
    y: f32,
    z: f32,
}

impl Position {
    pub fn from_str(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Player {
    character_name: String,
    name: String,
    position: Position,
}

impl Player {
    pub fn load(world: &World) -> Result<Vec<Self>, ZomboidWebMapError> {
        let conn = Connection::open(world.root.join("players.db"))?;
        let mut stmt = conn.prepare(
            r#"
        select username, name, x, y, z
        from networkPlayers;
            "#,
        )?;
        let players = stmt.query_map([], |row| {
            Ok(Player {
                name: row.get(0)?,
                character_name: row.get(1)?,
                position: Position::from_str(row.get(2)?, row.get(3)?, row.get(4)?),
            })
        })?;
        Ok(players
            .into_iter()
            .map(|x| x.unwrap())
            .collect::<Vec<Self>>())
    }
}
