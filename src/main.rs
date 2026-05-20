#![allow(warnings)]
use std::error::Error;

mod router;
mod setting;
mod api;
mod paseto;
mod argon2;
mod db;
mod version;
mod redis;

use crate::{setting::setting::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>>{


    let _ = match setting_server().await {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    Ok(())
}




