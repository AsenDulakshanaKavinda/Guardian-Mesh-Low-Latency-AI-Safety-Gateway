//! This module provides functionality for classifying threats using a gRPC client.
//! It defines the request and response structures for threat classification and implements
//! a function to communicate with the threat classification service.
//! The `threat_classification_client` function connects to the threat classification service,
//! sends a request with the input prompt string, and returns a response containing the threat label, category, and severity.
//! The module also handles errors using a custom `AppError` type defined in the `utils::errors` module.

use serde::{Deserialize, Serialize};
use tonic::Request;
use tonic::transport::Endpoint;

use crate::{
    generated::threat_proto::{
        ThreatClassifyRequest,
        threat_classification_service_client::ThreatClassificationServiceClient,
    },
    utils::errors::AppError,
};

#[derive(Debug, Serialize, Deserialize)] 
pub struct ThreatClassificationRequest {
    prompt_str: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThreatClassificationResponse {
    label: String,
    catagory: String,
    severity: String,
}

async fn threat_classification_client(
    prompt_str: &str,
) -> Result<ThreatClassificationResponse, AppError> {
    let endpoint = Endpoint::new("http://[::1]:10001").map_err(|e| AppError::Internal(e.into()))?;

    let channel = endpoint
        .connect()
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    let mut client = ThreatClassificationServiceClient::new(channel);

    let req = ThreatClassifyRequest {
        prompt_str: prompt_str.to_string(),
    };

    let request = Request::new(req);

    let response = client
        .classify_threat(request)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    let inner = response.into_inner();

    Ok(ThreatClassificationResponse {
        label: inner.label,
        catagory: inner.category,
        severity: inner.severity,
    })
}
