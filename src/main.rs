use std::fs;

use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::{get, post},
    Form, Router,
};
use mongodb::{bson::doc, options::ClientOptions, Client};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    name: String,
    email: String,
    password: String,
}

#[tokio::main]
async fn main() {
    let client_options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .unwrap();
    let client = Client::with_options(client_options).unwrap();

    let app = Router::new()
        .route("/", get(root))
        .route("/auth", post(auth))
        .with_state(client);
    let tcp = TcpListener::bind("localhost:8000").await.unwrap();
    axum::serve(tcp, app.into_make_service()).await.unwrap();
}
async fn root() -> impl IntoResponse {
    let file_content = fs::read_to_string("index.html").unwrap_or_default();
    Html(file_content)
}
async fn auth(State(client): State<Client>, Form(body): Form<User>) -> impl IntoResponse {
    let users = client.database("mydb").collection("users");
    let res = users
        .find_one(doc! {"email":body.email.clone()}, None)
        .await
        .unwrap();
    if res.is_some() {
        return Html("You are a user already".to_string());
    }
    users
        .insert_one(
            doc! {"name":body.name,"email": body.email,"password": body.password},
            None,
        )
        .await
        .unwrap();
    Html("You was not a user but now".to_string())
}
