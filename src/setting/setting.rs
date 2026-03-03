use std::error::Error;

use crate::router::router::router;

pub async fn setting_server() -> Result<(), Box<dyn Error>>{

    let ip_listener = "127.0.0.1:8000";

    let listen = tokio::net::TcpListener::bind(&ip_listener).await?;

    println!("Start IP: {}", &ip_listener);

    axum::serve(listen, router()).await?;

    Ok(())
}