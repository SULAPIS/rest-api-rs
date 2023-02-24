use aws_sdk_dynamodb::model::AttributeValue;
use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub user_id: String,
    pub email: String,
    pub password: String,
    pub user_name: String,
}

impl From<User> for Vec<AttributeValue> {
    fn from(user: User) -> Self {
        vec![
            AttributeValue::S(user.user_id),
            AttributeValue::S(user.email),
            AttributeValue::S(user.password),
            AttributeValue::S(user.user_name),
        ]
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserRegister {
    pub email: String,
    pub password: String,
    pub user_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Email {
    pub email: String,
}

impl From<UserRegister> for Vec<AttributeValue> {
    fn from(user: UserRegister) -> Self {
        vec![
            AttributeValue::S(user.email),
            AttributeValue::S(user.password),
            AttributeValue::S(user.user_name),
        ]
    }
}

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub user_id: String,
    pub exp: u64,
}

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}
