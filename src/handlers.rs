use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};

use crate::models::*;

pub async fn hello_world() -> &'static str {
    "Bienvenue sur mon API Axum !"
}

pub async fn ok() -> &'static str {
    "OK"
}

pub async fn greet(Path(user_name): Path<String>) -> String {
    format!("Bonjour, {user_name} !")
}

pub async fn calculator(Query(params): Query<Calculator>) -> String {
    format!("{} + {} = {}", params.a, params.b, params.a + params.b)
}

pub async fn products(State(state): State<AppState>) -> Json<Vec<Product>> {
    Json(state.lock().unwrap().clone())
}

pub async fn create_product(
    State(state): State<AppState>,
    Json(payload): Json<Product>,
) -> StatusCode {
    let mut products = state.lock().unwrap();
    if products.contains(&payload) {
        return StatusCode::CONFLICT;
    }
    products.push(payload);
    StatusCode::CREATED
}

pub async fn get_product(
    Path(product_id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<Product>, StatusCode> {
    let products = state.lock().unwrap();
    if let Some(product) = products.iter().find(|element| element.id == product_id) {
        Ok(Json(product.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn delete_product(Path(id): Path<i32>, State(state): State<AppState>) -> StatusCode {
    let mut products = state.lock().unwrap();
    if let Some(index) = products.iter().position(|element| element.id == id) {
        products.remove(index);
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
