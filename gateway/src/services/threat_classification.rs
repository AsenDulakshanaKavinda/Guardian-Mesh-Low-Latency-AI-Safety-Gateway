
use serde::{Deserialize, Serialize};
use tonic::Request;
use tonic::transport::Endpoint;

use crate::utils::errors::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct PPIMaskingRequest {
    prompt_str: String,
    masked_prompt_str: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PPIMaskingResponse {
    prompt_str: String,
    masked_prompt_str: String,
}
/*
async fn ppi_masking_clinet() -> Result<PPIMaskingResponse, AppError> {
    let endpoint = Endpoint::new("http://[::1]:10000")
        .map_err(|e| AppError::Internal(e.into()))?;

    let channel = endpoint
        .connect()
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    

    todo!();
}
 */