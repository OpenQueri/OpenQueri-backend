use std::error::Error;
use tower_http::cors::{CorsLayer};
use axum::http::{Method, header,HeaderValue};

use crate::router::router::router;

pub async fn setting_server() -> Result<(), Box<dyn Error + Send + Sync>>{
    
    let ip_listener = "127.0.0.1:8000";

    let listen = tokio::net::TcpListener::bind(&ip_listener).await?;

    println!("Start IP: {}", &ip_listener);

    axum::serve(listen, router().await).await?;

    Ok(())
}


pub fn cros() -> CorsLayer {
    println!("cros");
    CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap()) 
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCEPT,
        ])
        .allow_credentials(true)
}