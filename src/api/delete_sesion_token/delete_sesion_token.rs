

use axum::{Json,response::IntoResponse};
use serde_json::json;
use axum_extra::extract::cookie::{Cookie, CookieJar};



#[axum::debug_handler]
pub async fn delete_sesion_token(paseto: CookieJar) -> impl IntoResponse {


    let cookie = Cookie::build(("auth_token", ""))
        .path("/")
        .max_age(time::Duration::ZERO)
        .build();

    (
        paseto.add(cookie), 
        Json(json!({ "success_token_delete": true}))
    ).into_response()
}
