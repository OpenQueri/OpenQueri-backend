

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

    let _ = match db.create_user(&params.username, &hesh_pasword, &params.email, "User").await{
        Ok(id) => id,
        Err(e) => return {
            (jar, error_register("Error crate user").await).into_response()}
    };



    (
        Json(json!({ "success": true}))
    ).into_response()
}

pub async fn error_register(text: &str) -> Json<serde_json::Value> {
        Json(json!({

                "data": &text,
                "success": false,
            }))

}