use axum::serve;
use tokio::net::TcpListener;

mod handlers;
mod models;
mod router;

use crate::router::app;
use std::{
    io::Error,
    sync::{Arc, Mutex},
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let state = Arc::new(Mutex::new(Vec::new()));

    let app = app(state);

    let tcp: TcpListener = TcpListener::bind("0.0.0.0:3000").await?;

    println!("Lancé");

    serve(tcp, app).await?;

    Ok(())
}
