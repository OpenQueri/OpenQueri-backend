
use std::error::Error;
use crate::api::stats::redis_stats::RedisStats;
use axum::extract::ws::close_code::STATUS;
use axum::http::StatusCode;
use axum::{Json,response::IntoResponse};
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
};
use tokio::time::{sleep, Duration};
use axum_extra::extract::cookie::{CookieJar, Cookie};


pub async fn workspace_ws(
    jar: CookieJar, 
    ws: WebSocketUpgrade
) -> impl IntoResponse {
   
    let token = jar.get("auth_token").map(|cookie| cookie.value().to_string());

    if let Some(jwt_token) = token {
        println!("WS: Користувач авторизований, токен: {}", jwt_token);
        ws.on_upgrade(move |socket| workspace_socket(socket, jwt_token))
    } else {
        println!("WS: Спроба входу без токена!");
        StatusCode::UNAUTHORIZED.into_response()
    }
}

async fn workspace_socket(mut socket: WebSocket, token: String) {


    while let Some(Ok(msg)) = socket.recv().await {
        if let Message::Text(text) = msg {
            println!("Отримано від клієнта: {}", text);
        }
    }
}