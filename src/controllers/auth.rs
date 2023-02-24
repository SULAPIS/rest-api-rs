use std::sync::Arc;

use crate::{
    error::AppError,
    models::{
        self,
        auth::{Claims, User, UserLogin},
    },
    utils::{generate_token, get_timestamp_8_hours_from_now},
    DynamodbInfo, KEYS,
};
use aws_sdk_dynamodb::model::{AttributeValue, Select};
use axum::{extract::State, Json};
use axum_macros::debug_handler;
use jsonwebtoken::{encode, Header};
use serde_dynamo::aws_sdk_dynamodb_0_24::from_items;
use serde_json::{json, Value};
#[debug_handler]
pub async fn login(
    State(dynamodb): State<Arc<DynamodbInfo>>,
    Json(user_login): Json<UserLogin>,
) -> Result<Json<Value>, AppError> {
    let client = &dynamodb.client;
    let table_name = &dynamodb.table_name;

    println!("{:#?}", user_login);
    println!("{}", table_name);

    let token = generate_token(&user_login.email);

    let query_result = client
        .query()
        .table_name(table_name)
        .key_condition_expression("#key = :value".to_string())
        .expression_attribute_names("#key".to_string(), "userId".to_string())
        .expression_attribute_values(":value".to_string(), AttributeValue::S(token))
        .select(Select::AllAttributes)
        .send()
        .await
        .map_err(|err| AppError::from(&err))?;
    if let Some(user) = query_result.items {
        let user: Vec<User> = from_items(user).map_err(|_err| AppError::InternalServerError)?;
        if user.len() < 1 {
            return Err(AppError::UserDoeNotExist);
        }

        if user[0].password != user_login.password {
            return Err(AppError::WrongCredential);
        }

        let claims = Claims {
            user_id: user[0].user_id.to_owned(),
            exp: get_timestamp_8_hours_from_now(),
        };
        let token = encode(&Header::default(), &claims, &KEYS.encoding)
            .map_err(|_err| AppError::TokenCreation)?;

        Ok(Json(json!({ "access_token": token, "type": "Bearer" })))
    } else {
        Err(AppError::UserDoeNotExist)
    }
}

#[debug_handler]
pub async fn register(
    State(dynamodb): State<Arc<DynamodbInfo>>,
    Json(user): Json<models::auth::UserRegister>,
) -> Result<Json<Value>, AppError> {
    if user.email.is_empty() || user.password.is_empty() || user.user_name.is_empty() {
        return Err(AppError::MissingCredential);
    }
    let token = generate_token(&user.email);
    let item: Vec<AttributeValue> = user.into();

    let client = &dynamodb.client;
    let table_name = &dynamodb.table_name;

    client
        .put_item()
        .table_name(table_name)
        .item("userId".to_string(), AttributeValue::S(token))
        .item("email".to_string(), item[0].to_owned())
        .item("password".to_string(), item[1].to_owned())
        .item("userName".to_string(), item[2].to_owned())
        .send()
        .await
        .map_err(|err| AppError::from(&err))?;

    Ok(Json(json!({ "msg": "registered successfully" })))
}
