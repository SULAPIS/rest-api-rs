pub async fn route_info() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "routes": ["/", "/register", "/login", "/user_profile","/send_email_verify"],
        "routes_info": {
            "/" : "this route",
            "/register": "register a user with email and password",
            "/login": "login with the credentials used for registering",
            "/user_profile": "view your user profile with the token recieved from /login",
            "/send_email_verify": "send a verification email to the email used for registering"
        }
    }))
}
