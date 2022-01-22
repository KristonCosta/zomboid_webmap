use crate::util::Directory;
use crate::zomboid::World;
use futures_util::{SinkExt, StreamExt};
use structopt::StructOpt;
use tokio::time;
use warp::{ws::WebSocket, Filter};

use std::{sync::Arc, time::Duration};

mod util;
mod zomboid;

static REFRESH_DELAY_SECONDS: u64 = 5;

#[derive(StructOpt, Debug)]
#[structopt(name = "webmap")]
struct Opt {
    #[structopt(short, long)]
    world_name: String,
}

#[derive(Debug)]
pub enum ZomboidWebMapError {
    SqliteError(rusqlite::Error),
    SendError(std::sync::mpsc::SendError<String>),
}

impl From<rusqlite::Error> for ZomboidWebMapError {
    fn from(e: rusqlite::Error) -> Self {
        Self::SqliteError(e)
    }
}

pub struct Config {
    server_directory: Directory,
    world_name: String,
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();

    let server_directory = dirs::home_dir()
        .expect("Couldn't get home directory.")
        .join("Zomboid");

    let config = Config {
        server_directory: Directory::new(server_directory.to_str().unwrap()),
        world_name: opt.world_name,
    };

    let world = Arc::new(World::new(&config));
    let world = warp::any().map(move || world.clone());

    let socket = warp::path("connect")
        .and(warp::ws())
        .and(world)
        .map(|ws: warp::ws::Ws, world| ws.on_upgrade(move |socket| connected(socket, world)));

    let index = warp::path::end()
        .and(warp::get())
        .and(warp::fs::file("index.html"));

    let routes = index.or(socket).or(warp::fs::dir("static"));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn connected(ws: WebSocket, world: Arc<World>) {
    let mut world = (*world).clone();
    let (mut ws_tx, ws_rx) = ws.split();
    let mut interval = time::interval(Duration::from_secs(REFRESH_DELAY_SECONDS));
    tokio::task::spawn(async move {
        loop {
            interval.tick().await;
            let message = serde_json::to_string(&world.load_players().unwrap()).unwrap();
            match ws_tx.send(warp::ws::Message::text(message)).await {
                Err(_) => {
                    println!("User disconnected!");
                    break;
                }
                _ => {}
            }
        }
    });
}
