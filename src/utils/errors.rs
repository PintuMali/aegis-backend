use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sea_orm::DbErr),
    
    #[error("AWS DynamoDB error: {0}")]
    DynamoDB(#[from] aws_sdk_dynamodb::Error),
    
    #[error("AWS S3 error: {0}")]
    S3(#[from] aws_sdk_s3::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Not found")]
    NotFound,
    
    #[error("Unauthorized")]
    Unauthorized,
    
    #[error("Internal server error")]
    InternalServerError,
}
