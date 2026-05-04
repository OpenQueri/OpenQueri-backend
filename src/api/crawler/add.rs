use axum::{Json, extract::Query};
use serde::{Deserialize};
use serde_json::json;
use axum::{response::IntoResponse};

use crawler_engine::link_scrap;
use query_search::{add_data, DataADD};

#[derive(Debug, Deserialize)]
pub struct TextParam{
    url: String,
}

#[axum::debug_handler]
pub async fn add_parse_site_crawler(Json(params): Json<TextParam>) -> impl IntoResponse {

    let depht  = 1; 

    tokio::spawn(async move {
            let data_site_responses = match link_scrap(&params.url, depht).await {
            Ok(res) => res,
            Err(e) => { 
                eprintln!("❌ Помилка скрапінгу {}: {}", params.url, e);
                return ;
            },
        };


        for fragment in data_site_responses.iter() {
            
            let text_string = fragment.text.join(" ");

            let data = DataADD { 
                title: fragment.title.as_str(),
                link:  params.url.as_str(),
                text:  text_string.as_str(),
            };
            println!("Site {}", data.link);
            match add_data(&data).await {
                Ok(_) => (),
                Err(e) => {
                    println!("fn add_parse_site_crawler Error: {}", e);
                }
            };
        }
    });

    

    
    Json(serde_json::json!({
        "success": true, 
        "error": "Скрапінг запущено у фоні. Це займе деякий час."
    })).into_response()

}
