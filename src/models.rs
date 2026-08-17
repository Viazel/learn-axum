use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

pub type AppState = Arc<Mutex<Vec<Product>>>;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price: u32,
}

#[derive(Deserialize)]
pub struct Calculator {
    pub a: i32,
    pub b: i32,
}
