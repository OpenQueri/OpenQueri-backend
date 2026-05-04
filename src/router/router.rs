use std::error::Error;
use axum::{Router, routing::get,routing::post};
use crate::setting::setting::cros;
use query_search::{loading_data};
use crawler_engine::CheckingUniquenessLink::checking::load_links;

use crate::api::search::search::search_site_main;
use crate::api::crawler::add::add_parse_site_crawler;

pub async fn router() -> Router{

    println!("Start");

  
    match load_bin().await {
            Ok(_) => println!("Base data loaded!"),
            Err(e) => println!("{}",e),
        };


    println!("Finish");

   Router::new()
   .route("/search", get(search_site_main))
   .route("/parse-link", post(add_parse_site_crawler))
    .layer(cros())
}


pub async fn load_bin() -> Result<(), Box<dyn Error>>{

    loading_data().await?;

    load_links().await?; 
    
    Ok(())

}


