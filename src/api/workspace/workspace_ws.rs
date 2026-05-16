
use axum::http::StatusCode;
use axum::{Json,response::IntoResponse};
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
};
use chrono::format::parse;
use tokio::time::{sleep, Duration};
use axum_extra::extract::cookie::{CookieJar, Cookie};
use crate::paseto::paseto::PasetoAuth;
use serde_json::json;
use crate::version::VersionServer;
use serde::Deserialize;
use futures_util::{SinkExt, StreamExt};
use crate::db::db::{Db, TrackedSite};
use crate::redis::redis::RedisListener;
use std::result::Result::{Ok, Err};
pub struct WsWorkSpace {
    sender: futures_util::stream::SplitSink<WebSocket, Message>,
    receiver: futures_util::stream::SplitStream<WebSocket>,
    db: Db,
    user_info: User,
}
#[derive(Debug)]
struct User{
   username: String,
   role: String,
   id: String,
   email: String,
}


#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
enum ClientRequest {
    #[serde(rename = "SUBMIT_NEW_URL")]
    SubmitNewUrl {
        url: String,
    },
    #[serde(rename = "UPDATE_SITE")]
    UpdateSite {},
    #[serde(rename = "DELETE_URL")]
    DeleteUrl {
        url: String
    },
    #[serde(rename = "UPDATE_PENDING")]
    UpdatePeding {
        url: String,
        id: String
    },

}

impl WsWorkSpace {


    pub async fn new(mut socket: WebSocket, id: String) -> Self{
        let (mut send, mut rec) = socket.split();
        let db = Db::new().await;

        
        
        Self {
            sender: send,
            receiver: rec,
            db: db,
            user_info: User { username: "_".to_string(), role: "_".to_string(), id: id, email: "_".to_string() },
        }
    }


    pub async fn workspace_socket(&mut self){

        let email = match self.db.get_email_by_id(&self.user_info.id).await {
            Ok(Some(res)) => res,
            Ok(None) => r":\??".to_string(),
            Err(e) => {
                println!("{}", e);
                r":\??".to_string()
            }
        };

        self.user_info.email = email;


        let name_by_id = match self.db.get_user_name_by_id(&self.user_info.id).await {
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

        let role_user = match self.db.get_role_name_by_id(&self.user_info.id).await {
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

        self.user_info.role = role_user;
        self.user_info.username = name_by_id;

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
            ClientRequest::UpdateSite {} => {
                self.update_site().await;
            }
            ClientRequest::DeleteUrl {url} => {
                self.sumbit_delete_url(url.as_str()).await;
            }
            ClientRequest::UpdatePeding { url, id } => {
                self.update_url_peding(&url, &id).await;
            }
        }
    }

    async fn update_url_peding(
        &mut self,
        url_data: &str,
        id: &str
    ){


        if self.user_info.role == "Admin" {

            match self.db.update_peding(&url_data, "crawling").await {
                Ok(_) => {
                    let response = json!({
                        "type": "UPDATE_PENDING", 
                        "status": "crawling",
                        "id": id,
                    });


                    let _ = self.sender.send(Message::Text(response.to_string().into())).await;

                }
                Err(e) => {
                    println!("{}",e);
                }
                
            }

            
        }
        

    }


    async fn sumbit_delete_url(
        &mut self,
        url_data: &str
    ){
        
        match self.db.get_id_by_url(url_data).await {
            Ok(id) => {
                let id = match id {
                    Some(ok) => ok,
                    None => -1,

                };
                if self.user_info.id == id.to_string() || self.user_info.role == "Admin" {
                
                    self.db.delete_url_crawler_user(url_data).await;
                }
            }
            Err(e) => {
                println!("sumbit_delete_url: {}", e);
            }
            
        };
    }

    async fn sumbit_new_url(
        &mut self,
        url_data: &str
    ){

        let id = &self.parse(self.user_info.id.clone()).await;

        let state = match &self.db.create_new_url(*id, url_data, &self.user_info.username).await {
            Ok(state) => state.clone(),
            Err(e_status) => (format!("Error {}",e_status).to_string(),format!("Error").to_string()),
        };


        let response = json!({
            "type": "TrackedSite", 
            "url": url_data,
            "status": state.0,
            "submittedBy": self.user_info.username,
            "lastUpdate": state.1
        });
        
        let _ = self.sender.send(Message::Text(response.to_string().into())).await;

    }

    pub async fn update_site(&mut self){
        let error_update = vec![TrackedSite{
            url: "Error _-_ update_site don't work".to_string(),
            status: "Error".to_string(),
            submitted_by: "None".to_string(),
            last_update: "00.00.0000".to_string()
        }];
        let vec_site ;
        match self.user_info.role.as_str() {
            
            "Admin" => {
                vec_site = match self.db.get_crawler_list_admin().await {
                    Ok(vec) => vec,
                    Err(e) => {
                        println!("update_site {}",e);
                        error_update
                    }
                };
            },
            "User" => {
                println!("User");
                let id_num = self.parse(self.user_info.id.clone()).await;
                println!("{}",id_num);
                vec_site = match self.db.get_crawler_list_user(id_num).await {
                    Ok(vec) => vec,
                    Err(e) => {
                        println!("update_site {}",e);
                        error_update
                    }
                };

            },
            _ => {
                vec_site = error_update;
            },
            
        }

        println!("{:?}", vec_site);

        for site in vec_site.iter() {

            let response = json!({
                "type": "TrackedSite", 
                "url": site.url,
                "status": site.status,
                "submittedBy": site.submitted_by,
                "lastUpdate": site.last_update,
            });
        
            let _ = &self.sender.send(Message::Text(response.to_string().into())).await;
            
        }
    }



    pub async fn parse(&self, id: String) -> i32 {
        match id.parse::<i32>() {
            Ok(id) => return id,
            Err(e) => return -1,
        };
    }
}