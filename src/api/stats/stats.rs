use std::error::Error;
use crate::api::stats::redis_stats::RedisStats;
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
};
use tokio::time::{sleep, Duration};



pub async fn stats_ws(ws: WebSocketUpgrade) -> impl axum::response::IntoResponse{
    ws.on_upgrade(stats_socket)
}


async fn stats_socket(mut socket: WebSocket)  {
    
    let redis = RedisStats::new();


    
    loop {

        let get_cout = redis.get_stats().await;

        let msg = Message::Text(format!("Счётчик запросов: {}", get_cout ).into());

        if socket.send(msg).await.is_err() {
            println!("Клиент отвалился");
            break; 
        }
    }
    
   
}