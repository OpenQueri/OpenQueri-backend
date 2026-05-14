
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


pub struct WsWorkSpace {
    sender: futures_util::stream::SplitSink<WebSocket, Message>,
    receiver: futures_util::stream::SplitStream<WebSocket>,
    id: String,
    db: Db,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
enum ClientRequest {
    #[serde(rename = "SUBMIT_NEW_URL")]
    SubmitNewUrl {
        url: String,
    },
}

impl WsWorkSpace {


    pub async fn new(mut socket: WebSocket, id: String) -> Self{
        let (mut send, mut rec) = socket.split();
        let db = Db::new().await;
        Self {
            sender: send,
            receiver: rec,
            id: id,
            db: db,
        }
    }


    pub async fn workspace_socket(&mut self){

        let name_by_id = match self.db.get_user_name_by_id(&self.id).await {
            Ok(ok_option) => match ok_option {
                Some(res) => res,
                None => {
                    r":\??".to_string()
                },
            }
            Err(e) => {
                println!("{}",e);
                r":\??".to_string()
            }
        };

        let role_user = match self.db.get_role_name_by_id(&self.id).await {
            Ok(role) => match role {
                Some(role) => role,
                None => "User".to_string(),
            }
            Err(e) => {
                println!("{}",e);
                "User".to_string()
            }
        };

        let initial_msg = json!({
            "version": VersionServer::VERSION_SERVER,
            "username": name_by_id,
            "role": role_user,
        });

        if let Ok(json_string) = serde_json::to_string(&initial_msg) {
            let _ = self.sender.send(Message::Text(json_string.into())).await;
        }


        while let Some(Ok(msg)) = self.receiver.next().await {
            if let Message::Text(text) = msg {
                println!("WS Received: {}", text);

                match serde_json::from_str::<ClientRequest>(&text) {
                    Ok(request) => {
                        self.handle_client_message(request).await;
                    }
                    Err(e) => {
                        eprintln!("WS JSON Parse Error: {} | Data: {}", e, text);
                        let _ = self.sender.send(Message::Text(json!({"error": "Unknown command"}).to_string().into())).await;
                    }
                }
            } else if let Message::Close(_) = msg {
                break; 
            }
        }

    }


    async fn handle_client_message(
        &mut self,
        request: ClientRequest
    ) {
        match request {
            ClientRequest::SubmitNewUrl { url } => {
                self.sumbit_new_url(&url).await;
            }
        }
    }

    async fn sumbit_new_url(
        &mut self,
        url_data: &str
    ){

        println!("{}",url_data);


        let response = json!({
            "type": "arr_quesion", 
            "url": url_data,
            "status": "review",
            "submittedBy": "System",
            "lastUpdate": "Щойно"
        });
        
        let _ = self.sender.send(Message::Text(response.to_string().into())).await;

    }
}