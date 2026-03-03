
use std::error::Error;

mod router;
mod setting;
mod api;

use crate::setting::setting::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{


    let _ = match setting_server().await {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    Ok(())
}




