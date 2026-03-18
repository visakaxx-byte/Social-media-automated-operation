use crate::web::state::AppState;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsMessage {
    #[serde(rename = "ping")]
    Ping,
    #[serde(rename = "pong")]
    Pong,
    #[serde(rename = "task_update")]
    TaskUpdate {
        task_id: String,
        status: String,
        progress: Option<u32>,
    },
    #[serde(rename = "log")]
    Log {
        level: String,
        message: String,
        timestamp: String,
    },
    #[serde(rename = "account_status")]
    AccountStatus {
        account_id: String,
        status: String,
        health_score: i32,
    },
    #[serde(rename = "stats_update")]
    StatsUpdate {
        pending_tasks: usize,
        running_tasks: usize,
        active_accounts: usize,
    },
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(_state): State<AppState>,
) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(socket: WebSocket) {
    let (mut sender, mut receiver) = socket.split();

    // Spawn a task to send periodic updates
    let mut send_task = tokio::spawn(async move {
        loop {
            // Send ping every 30 seconds
            tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;

            let msg = WsMessage::Ping;
            if let Ok(json) = serde_json::to_string(&msg) {
                if sender.send(Message::Text(json)).await.is_err() {
                    break;
                }
            }
        }
    });

    // Handle incoming messages
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                if let Ok(ws_msg) = serde_json::from_str::<WsMessage>(&text) {
                    match ws_msg {
                        WsMessage::Ping => {
                            tracing::debug!("Received ping from client");
                        }
                        _ => {
                            tracing::debug!("Received message: {:?}", ws_msg);
                        }
                    }
                }
            }
        }
    });

    // Wait for either task to finish
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }

    tracing::info!("WebSocket connection closed");
}
