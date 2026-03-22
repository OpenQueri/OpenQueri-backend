use std::error::Error;


use crate::router::router::router;
use axum::http::Method;
use tower_http::cors::{CorsLayer, Any};

pub async fn setting_server() -> Result<(), Box<dyn Error>>{
    
    let ip_listener = "127.0.0.1:8000";

    let listen = tokio::net::TcpListener::bind(&ip_listener).await?;

    println!("Start IP: {}", &ip_listener);

    axum::serve(listen, router().await).await?;

    Ok(())
}


pub fn cros() -> CorsLayer{
    println!("cros");
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_headers(Any)
}