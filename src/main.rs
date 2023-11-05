use axum::{
    body,
    extract::Path,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use http::{header, StatusCode};
use include_dir::{include_dir, Dir};
use std::net::SocketAddr;

static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/assets");

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/*path", get(serve_asset));

    let addr = SocketAddr::from(([0, 0, 0, 0], 5000));
    tracing::debug!("listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn serve_asset(Path(path): Path<String>) -> impl IntoResponse {
    let path = path.trim_start_matches('/');

    match STATIC_DIR.get_file(path) {
        Some(file) => Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                header::HeaderValue::from_str(mime::IMAGE_JPEG.essence_str()).unwrap(),
            )
            .body(body::boxed(body::Full::from(file.contents())))
            .unwrap(),
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(body::boxed(body::Empty::new()))
            .unwrap(),
    }
}
