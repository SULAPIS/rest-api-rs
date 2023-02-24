use std::{ops::Not, sync::Arc};

use crate::{error::AppError, models::auth::Email, SNSInfo};
use axum::{extract::State, Json};
use axum_macros::debug_handler;
use serde_json::{json, Value};
#[debug_handler]
pub async fn send_email_verify(
    State(sns): State<Arc<SNSInfo>>,
    Json(email): Json<Email>,
) -> Result<Json<Value>, AppError> {
    let client = &sns.client;
    let rsp = client
        .subscribe()
        .topic_arn(&sns.topic_arn)
        .protocol("email")
        .endpoint(email.email)
        .send()
        .await
        .map_err(|err| AppError::from(&err))?;

    println!("{:#?}", rsp.subscription_arn());

    Ok(Json(json!({ "send": "ok" })))
}

#[debug_handler]
pub async fn if_email_verified(
    State(sns): State<Arc<SNSInfo>>,
    Json(email): Json<Email>,
) -> Result<Json<Value>, AppError> {
    let client = &sns.client;

    let rsp = client
        .list_subscriptions_by_topic()
        .topic_arn(&sns.topic_arn)
        .send()
        .await
        .map_err(|err| AppError::from(&err))?;

    let verified = rsp.subscriptions().unwrap().iter().any(|sub| {
        sub.endpoint().unwrap() == email.email
            && sub.protocol().unwrap() == "email"
            && sub
                .subscription_arn()
                .unwrap()
                .contains("PendingConfirmation")
                .not()
    });

    Ok(Json(json!({ "email": &email.email, "verified": verified })))
}
