use actix_web::{HttpResponse, Result, get};
use tracing::instrument;
use crate::utils::response::{ ZarynMessage, ZarynError };

// #[get("")]
// #[instrument]
pub async fn route_not_found() -> Result<HttpResponse, ZarynError> {
   Err(ZarynError::NotFound)
}



#[get("/health")]
#[instrument]
pub async fn health() -> HttpResponse {
    ZarynMessage::success(true, "I'm feeling Good".to_string())
}

