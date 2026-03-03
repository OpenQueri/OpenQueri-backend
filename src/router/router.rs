use crate::api::search::search::search;
use axum::{
    Router, routing::get
};

pub fn router() -> Router{
   Router::new()
   .route("/", get(index_page))
   .route("/api/search", get(search()))
}






async fn index_page() -> String{

    let result = "Hello World".to_string();
    result
}