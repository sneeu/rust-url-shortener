use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Json, Redirect},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

mod config;
mod data;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/create", post(create_url))
        .route("/inspect/:key", get(inspect_url))
        .route("/:key", get(get_url));

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Deserialize)]
struct CreateUrl {
    url: String,
}

#[derive(Debug, Serialize)]
struct CreateUrlResponse {
    url: String,
}

async fn create_url(Json(input): Json<CreateUrl>) -> impl IntoResponse {
    let url = data::create_url(input.url);

    let config = config::Config::read().unwrap();
    let host = config.host_name;

    match url {
        Ok(url) => Ok(Json(CreateUrlResponse {
            url: format!("{host}/{}", url.id),
        })),
        _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn inspect_url(Path(id): Path<i64>) -> Result<impl IntoResponse, StatusCode> {
    let url = data::read_url(id);

    match url {
        Ok(url) => Ok(Json(url)),
        _ => Err(StatusCode::NOT_FOUND),
    }
}

async fn get_url(Path(id): Path<i64>) -> Result<impl IntoResponse, StatusCode> {
    let url = data::read_url(id);
    let the_url = url.unwrap();

    Ok(Redirect::permanent(&the_url.url))
}
