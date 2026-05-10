

use axum::{Json,response::IntoResponse};
use serde::{Deserialize};
use serde_json::json;
use axum_extra::extract::cookie::{Cookie, CookieJar};

use crate::db::db::Db;
use crate::argon2::argon::Argon;
use crate::paseto::paseto::PasetoAuth;

#[derive(Debug, Deserialize)]
pub struct TextParam{
    email: String,
    password: String,
}


#[axum::debug_handler]
pub async fn login(jar: CookieJar, Json(params): Json<TextParam>) -> impl IntoResponse {


    let db = Db::new().await;

    let id = match db.get_user_id_by_email(&params.email).await {
        Ok(res) => match res {
            Some(res) => &format!("{}",res),
            None => return (jar, error_login("wrong email").await).into_response(),
        },

        Err(e) => return (jar, error_login(&format!("Error email: {}",e)).await).into_response(),
    };


    let password_hash= match db.get_user_hash_pasword_by_email(&params.email).await {
        Ok(res) => match res {
            Some(password_hash) => password_hash,
            None => return (jar, error_login("wrong password").await).into_response(),
        }
        Err(e) => return (jar, error_login(&format!("Error password_hash: {}",e)).await).into_response(),
    };

    let cookie;

    match Argon::verify_pwd(&password_hash, &params.password).await {
        Ok(bool) => match bool {
            true => {
                let token = match PasetoAuth::create_token(&id, 60 * 60 * 24).await {
                    Ok(token) => token,
                    Err(_) => return (jar, error_login("Error crate token").await).into_response(),
                    
                };
                cookie = Cookie::build(("auth_token", token))
                        .path("/")
                        .http_only(true)
                        .same_site(axum_extra::extract::cookie::SameSite::Lax)
                        .build();
            },
            false => return (jar, error_login("wrong password").await).into_response(),
        },
        Err(e) => return (jar, error_login(&format!("Error verify_pwd password: {}",e)).await).into_response(),
    };

    (
        jar.add(cookie), 
        Json(json!({ "success": true}))
    ).into_response()
}


pub async fn error_login(text: &str) -> Json<serde_json::Value> {
        Json(json!({

                "data": &text,
                "success": false,
            }))

}