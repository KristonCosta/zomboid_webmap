use arc_swap::{ArcSwap, ArcSwapAny};
use futures_util::{SinkExt, StreamExt};
use tokio::{
    sync::mpsc::{channel, Sender},
    time,
};
use warp::{ws::WebSocket, Filter};
use zomboid::{Player, PlayerDTO, State};

use std::{convert::Infallible, sync::Arc, time::Duration};

mod zomboid;

static REFRESH_DELAY_SECONDS: u64 = 1;

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

#[tokio::main]
async fn main() {
    let (sender, mut receiver) = channel::<Arc<PlayerDTO>>(200);

    let sender = Arc::new(sender);
    let sender = warp::any().map(move || sender.clone());
    let player = warp::path("player")
        .and(warp::query::<PlayerDTO>())
        .and(sender)
        .and_then(process_player_dto);

    let state = Arc::new(ArcSwap::from(Arc::new(State::new())));
    let inner_state = state.clone();
    tokio::task::spawn(async move {
        while let Some(player) = receiver.recv().await {
            let mut new_state = State::from(&inner_state.load());
            new_state.update_player(&player);
            inner_state.store(Arc::new(new_state));
        }
    });

    let state = warp::any().map(move || state.clone());
    let socket = warp::path("connect")
        .and(warp::ws())
        .and(state)
        .map(|ws: warp::ws::Ws, state| ws.on_upgrade(move |socket| connected(socket, state)));

    let index = warp::path::end()
        .and(warp::get())
        .and(warp::fs::file("index.html"));

    let routes = index.or(socket).or(player).or(warp::fs::dir("static"));

    warp::serve(routes).run(([127, 0, 0, 1], 12345)).await;
}

async fn process_player_dto(
    player: PlayerDTO,
    sender: Arc<Sender<Arc<PlayerDTO>>>,
) -> Result<impl warp::Reply, Infallible> {
    sender.send(Arc::new(player)).await.unwrap();
    Ok(format!("Success"))
}

async fn connected(ws: WebSocket, world: Arc<ArcSwapAny<Arc<State>>>) {
    let (mut ws_tx, _) = ws.split();
    let mut interval = time::interval(Duration::from_secs(REFRESH_DELAY_SECONDS));
    tokio::task::spawn(async move {
        loop {
            interval.tick().await;
            let world = (*world).load();
            let players = world
                .players()
                .values()
                .into_iter()
                .collect::<Vec<&Player>>();
            let message = serde_json::to_string(&players).unwrap();
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
