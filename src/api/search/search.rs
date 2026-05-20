

use serde::{Deserialize};
use axum::{Json, response::IntoResponse,extract::Query};

use query_search::search_data;
use crate::redis::redis_stats::RedisStats;
use serde_json::json;

use axum::extract::State;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct TextParam{
    text: String,
}

#[axum::debug_handler]
pub async fn search_site_main(Query(params): Query<TextParam>, State(redis): State<Arc<RedisStats>>) -> impl IntoResponse {

    let text_qwery = params.text;

    redis.add_cout().await;

    match search_data(&text_qwery).await {
        Ok(data) => {
            Json(json!({
                "success": true,
                "query": text_qwery,
                "results": data.meta_data,
                "duration_ms": data.duration.as_secs_f64() ,
                "length": text_qwery.len(),
            }))
        }
        Err(e) => {
            Json(json!({
                "success": false,
                "query": text_qwery,
                "error": format!("{}", e),
                "results": [],
            }))
        }
    }
        
}
