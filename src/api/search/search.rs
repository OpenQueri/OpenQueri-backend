

use axum::{Json, extract::Query};
use serde::{Deserialize};
use serde_json::json;

use query_search::search_data;

#[derive(Debug, Deserialize)]
pub struct TextParam{
    text: String,
}

#[axum::debug_handler]
pub async fn search_main(Query(params): Query<TextParam>) -> Json<serde_json::Value>{

    let text_qwery = params.text;


    

    match search_data(&text_qwery).await {
        Ok(data) => {
            Json(json!({
                "success": true,
                "language": data.language,
                "query": text_qwery,
                "results": data.result,
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