use actix_web::{HttpResponse, Result, get};
use tracing::instrument;
use crate::utils::response::{ ZarynResponse, ZarynError };

// #[get("")]
// #[instrument]
pub async fn route_not_found() -> Result<HttpResponse, ZarynError> {
   Err(ZarynError::NotFound)
}



#[get("/health")]
#[instrument]
pub async fn health() -> HttpResponse {
    ZarynResponse::success(true, None,Some("I'm feeling Good".to_string()))
}

