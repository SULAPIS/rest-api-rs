use std::sync::Arc;

use aws_sdk_dynamodb as dynamodb;
use axum::{async_trait, routing::get, routing::post, Router};
use once_cell::sync::Lazy;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

mod controllers;
mod error;
mod models;

mod utils;

static KEYS: Lazy<models::auth::Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "Your secret here".to_owned());
    models::auth::Keys::new(secret.as_bytes())
});
#[derive(Debug, Clone)]
pub struct DynamodbInfo {
    pub client: dynamodb::Client,
    pub table_name: String,
}

impl Database for DynamodbInfo {}
#[async_trait]
pub trait Database: Send + Sync {}

#[derive(Debug, Clone)]
pub struct SNSInfo {
    pub client: aws_sdk_sns::Client,
    pub topic_arn: String,
}
#[tokio::main]
async fn main() {
    let config = aws_config::load_from_env().await;
    let client = dynamodb::Client::new(&config);
    let sns_client = aws_sdk_sns::Client::new(&config);

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "axum_api=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let dynamo = Arc::new(DynamodbInfo {
        client,
        table_name: "user_demo".to_string(),
    });

    let sns = Arc::new(SNSInfo {
        client: sns_client,
        topic_arn: "arn:aws:sns:eu-north-1:716650317489:rust".to_string(),
    });

    let user_routes = Router::new()
        .route("/", get(controllers::info::route_info))
        .route("/login", post(controllers::auth::login))
        .route("/register", post(controllers::auth::register))
        .route("/user_profile", get(controllers::user::user_profile))
        .with_state(dynamo);

    let email_routes = Router::new()
        .route("/subscribe", post(controllers::sns::send_email_verify))
        .route("/check_verified", post(controllers::sns::if_email_verified))
        .with_state(sns);

    let app = Router::new()
        .nest("/user", user_routes)
        .nest("/email", email_routes);

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed");
}
