use axum::{Extension, Json, http::StatusCode};
use chrono::{NaiveDateTime, Utc};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{entities, models::APIResponse, utils::{errors::AppError, guards::CurrentUser}};

#[derive(Debug, Deserialize, Serialize)]
pub struct PromptData {
    prompt: String,
    prompt_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BadPromptResponse {
    prompt: String,
    prompt_type: String,
    reqested_at: NaiveDateTime
}

pub async fn insert_prompt(
    Extension(current_user): Extension<CurrentUser>,
    Extension(db): Extension<DatabaseConnection>,
    Json(prompt_data): Json<PromptData>
) -> Result<(StatusCode, Json<APIResponse<BadPromptResponse>>), AppError> {


    let user_id = current_user.0.user_id;
    let prompt_id =  Uuid::new_v4().to_string(); // todo - get the prompt id from the user request
    let requested_at = Utc::now().naive_utc();


    let new_prompt = entities::bad_prompt::ActiveModel {
        user_id: Set(user_id.to_owned()),
        prompt_id: Set(prompt_id.to_owned()),
        prompt: Set(prompt_data.prompt.to_owned()),
        r#type: Set(prompt_data.prompt_type.to_owned()),
        reqested_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    let inserted_prompt = new_prompt
        .insert(&db)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    let response = BadPromptResponse {
        prompt: prompt_data.prompt,
        prompt_type: prompt_data.prompt_type,
        reqested_at: requested_at
    };

    Ok((
        StatusCode::CREATED,
        Json(APIResponse {
            success: true,
            message: "bad prompt detected successfully".to_string(),
            data: Some(response),
        }),
    ))




}