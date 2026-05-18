//! This module defines the `ppi_masking` service,
//! which provides functionality for masking potentially sensitive information in text data.
//! It includes a request and response structure for handling PII masking requests,
//! as well as a client function that interacts with an external Named Entity Recognition (NER) service
//! to extract entities from the input text and return a masked version of the prompt string.
//! The service uses gRPC for communication and handles errors using a custom `AppError` type defined in the `utils::errors` module.

use serde::{Deserialize, Serialize};
use tonic::Request;
use tonic::transport::Endpoint;

use crate::generated::ner::NerRequest;

use crate::generated::ner::ner_service_client::NerServiceClient;
use crate::utils::errors::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct PPIMaskingRequest {
    prompt_str: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PPIMaskingResponse {
    prompt_str: String,
    masked_prompt_str: String,
}

async fn ppi_masking_clinet(prompt_str: &str) -> Result<PPIMaskingResponse, AppError> {
    let endpoint = Endpoint::new("http://[::1]:10000").map_err(|e| AppError::Internal(e.into()))?;

    let channel = endpoint
        .connect()
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    let mut client = NerServiceClient::new(channel);

    let req = NerRequest {
        prompt_str: prompt_str.to_string(),
    };

    let request = Request::new(req);

    let response = client
        .extract_entities(request)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    let inner = response.into_inner();

    Ok(PPIMaskingResponse {
        prompt_str: prompt_str.to_string(),
        masked_prompt_str: format!("{:?}", inner.entities), // placeholder mapping
    })
}
