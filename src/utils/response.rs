use crate::models::wallet::Wallet;
use serde::{Serialize, Deserialize};
use actix_web::{HttpResponse,  dev::HttpResponseBuilder, error, http::StatusCode};
use derive_more::{Display, Error};

#[derive(Serialize, Deserialize)]
pub struct ListZarynResponse {
    pub status: bool,
    pub data: Option<Vec<Wallet>>,
    pub message: Option<String>,
}
impl ListZarynResponse {
    pub fn success(status: bool, data:Option<Vec<Wallet>>,message: Option<String>) -> HttpResponse { 
       HttpResponse::Ok().json(
            ListZarynResponse {
                status,
                data,
                message,
           }
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct ZarynResponse {
    pub status: bool,
    pub data: Option<Wallet>,
    pub message: String,
}
impl ZarynResponse {
    pub fn success(status: bool, data:Option<Wallet>,message: String) -> HttpResponse {
        HttpResponse::Ok().json(
            ZarynResponse {
                status,
                data,
                message,
           }
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct ZarynMessage {
    pub status: bool,
    pub message: String,
}
impl ZarynMessage {
    pub fn success(status: bool, message: String) -> HttpResponse {
        HttpResponse::Ok().json(
            ZarynMessage {
                status,
                message,
           }
        )
    }
}

#[derive(Serialize, Deserialize)]
struct ErrorMessage { 
    pub status: bool, 
    pub status_code: u16, 
    pub message: String 
} impl ErrorMessage {
    fn message(status: bool, status_code: u16, message: String) -> ErrorMessage {
        ErrorMessage {
            status,
            status_code,
            message
          }
    }
}
#[derive(Debug, Display, Error)]
pub enum ZarynError {
    #[display(fmt = "Message: {}", field)]
    ValidationError { field: String },

    #[display(fmt = "Something went wrong, Please try again later.")]
    InternalError,

    #[display(fmt = "You sent a bad request")]
    BadClientData,

    #[display(fmt = "url not found, check your input again")]
    NotFound,

    #[display(fmt = "Wallet not found")]
    WalletNotFound,

    #[display(fmt = "Wallet doesn't have enough balance")]
    NotEnoughBalance,
    
    #[display(fmt = "Duplicate Wallet details found, Wallet can't be created")]
    ErrorDuplicateWalletFound,

    #[display(fmt = "Transaction not found")]
    TransactionNotFound,

    #[display(fmt = "Unable to process this transaction, try again")]
    TransactionNotProcessed,

    // #[display(fmt = "auth")]
    // TokenNotFound,

}

impl error::ResponseError for ZarynError {
     fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code()).json(ErrorMessage::message(false, self.status_code().as_u16(), self.to_string())
        )
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            ZarynError::ValidationError { .. } => StatusCode::UNAUTHORIZED,
            ZarynError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ZarynError::BadClientData => StatusCode::BAD_REQUEST,
            ZarynError::NotEnoughBalance => StatusCode::BAD_REQUEST,
            ZarynError::NotFound => StatusCode::NOT_FOUND,
            
            ZarynError::TransactionNotProcessed => StatusCode::NOT_ACCEPTABLE,
            ZarynError::TransactionNotFound => StatusCode::NOT_FOUND,

            ZarynError::WalletNotFound => StatusCode::NOT_FOUND,
            ZarynError::ErrorDuplicateWalletFound => StatusCode::NOT_ACCEPTABLE,

            // ZarynError::TokenNotFound => StatusCode::NON_AUTHORITATIVE_INFORMATION
        }
    }
}