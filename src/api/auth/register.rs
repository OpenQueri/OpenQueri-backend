

use axum::{Json, response::IntoResponse};
use serde::{Deserialize};
use serde_json::json;
use crate::db::db::Db;
use crate::argon2::argon::Argon;
use crate::paseto::paseto::PasetoAuth;
use axum_extra::extract::cookie::{Cookie, CookieJar};

#[derive(Debug, Deserialize)]
pub struct TextParam{
    email: String,
    username: String,
    password: String,
}


#[axum::debug_handler]
pub async fn register(jar: CookieJar, Json(params): Json<TextParam>) -> impl IntoResponse  {

    let db = Db::new().await;

    let hesh_pasword = Argon::hash_pwd(&params.password).await;

    let id = match db.create_user(&params.username, &hesh_pasword, &params.email).await{
        Ok(id) => id,
        Err(_) => return (jar, error_register("Error crate user").await).into_response(),
    };

    let id = format!("{}",id);
    let token = match PasetoAuth::create_token(&id, 60 * 60 * 24).await {
        Ok(token) => token,
        Err(_) => return (jar, error_register("Error crate token").await).into_response(),
        
    };

    let cookie = Cookie::build(("auth_token", token))
        .path("/")
        .http_only(true)
        .same_site(axum_extra::extract::cookie::SameSite::Lax)
        .build();

    (
        jar.add(cookie), 
        Json(json!({ "success": true}))
    ).into_response()
}

pub async fn error_register(text: &str) -> Json<serde_json::Value> {
        Json(json!({

                "data": &text,
                "success": false,
            }))

}