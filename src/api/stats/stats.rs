use std::error::Error;
use crate::redis::redis_stats::RedisStats;
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
};
use tokio::time::{sleep, Duration};



pub async fn stats_ws(ws: WebSocketUpgrade) -> impl axum::response::IntoResponse{
    ws.on_upgrade(stats_socket)
}


async fn stats_socket(mut socket: WebSocket)  {
    
    let redis = RedisStats::new();

    let mut conn = match redis.client.as_ref().unwrap().get_multiplexed_async_connection().await {
        Ok(c) => c,
        Err(_) => return, 
    };

    
    loop {

        let key = "stats:counter_requests";
        let get_cout: u32 = match redis::cmd("GET").arg(key).query_async(&mut conn).await {
            Ok(val) => val,
            Err(_) => 0,
        };

        let msg = Message::Text(format!("{}", get_cout).into());

        if socket.send(msg).await.is_err() {
            break; 
        }


        sleep(Duration::from_millis(1000)).await;
    }
    
   
}