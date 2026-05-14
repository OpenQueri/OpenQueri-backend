
use axum::http::StatusCode;
use axum::{Json,response::IntoResponse};
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
};
use tokio::time::{sleep, Duration};
use axum_extra::extract::cookie::{CookieJar, Cookie};
use crate::paseto::paseto::PasetoAuth;
use serde_json::json;
use crate::version::VersionServer;
use serde::Deserialize;
use futures_util::{SinkExt, StreamExt};

use crate::db::db::Db;
use crate::api::workspace::workspace_ws::WsWorkSpace;



pub async fn workspace_ws(
    jar: CookieJar, 
    ws: WebSocketUpgrade
) -> impl IntoResponse {
   
    let token = jar.get("auth_token").map(|cookie| cookie.value().to_string());
    
    if let Some(token) = token {

        match PasetoAuth::verify_token(&token).await {
            Ok(id) => {
                ws.on_upgrade(move |socket| async move {
                    let mut ws_session = WsWorkSpace::new(socket, id).await;
                    ws_session.workspace_socket().await;
                }) 
            },
            Err(_e) => {
                (StatusCode::UNAUTHORIZED, "Unauthorized").into_response()
            },
        }
        
        
    } else {
        (StatusCode::UNAUTHORIZED, "Unauthorized").into_response()
    }
}
