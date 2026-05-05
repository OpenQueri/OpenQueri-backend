use std::error::Error;
use axum::{Router, routing::get,routing::post};
use crate::setting::setting::cros;
use query_search::{loading_data};
use crawler_engine::CheckingUniquenessLink::checking::load_links;

use crate::api::search::search::search_site_main;
use crate::api::crawler::add::add_parse_site_crawler;
use crate::api::stats::stats::stats_ws;

use std::sync::Arc;
use crate::api::stats::redis_stats::RedisStats;

pub async fn router() -> Router{

    println!("Start");

    let redis_stats = Arc::new(RedisStats::new()); 

  
    match load_bin().await {
            Ok(_) => println!("Base data loaded!"),
            Err(e) => println!("{}",e),
        };


    println!("Finish");

   Router::new()
   .route("/search", get(search_site_main))
   .route("/parse-link", post(add_parse_site_crawler))
   .route("/stats-ws", get(stats_ws))
   .with_state(redis_stats)
    .layer(cros())
}


pub async fn load_bin() -> Result<(), Box<dyn Error>>{

    loading_data().await?;

    load_links().await?; 
    
    Ok(())

}


