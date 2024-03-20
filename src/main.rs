pub mod api;
pub mod error;

use std::net::SocketAddr;
use std::path::PathBuf;

use api::Project;
use axum::response::IntoResponse;
use axum::{
    body::Body,
    extract::{Path, State},
    http::Request,
    response::Response,
    routing::get,
    Router,
};
use clap::{arg, command, Parser};
use tower::util::ServiceExt;
use tower_http::services::ServeFile;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    host: SocketAddr,
    #[arg(long, default_value = "view/build")]
    assets: PathBuf,
    #[arg(short, long, default_value = "files/")]
    resources: PathBuf,
}

#[tokio::main]
async fn main() {
    let Args {
        host,
        assets,
        resources,
    } = Args::parse();

    let project = Project::new(resources.clone());

    let app = Router::new()
        .nest("/api", api::routes(project))
        .route("/", get(static_index).with_state(assets.clone()))
        .route("/files/*file", get(static_assets).with_state(resources))
        .route("/*file", get(static_assets).with_state(assets));

    let listener = tokio::net::TcpListener::bind(host).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn static_index(State(dir): State<PathBuf>, req: Request<Body>) -> Response {
    ServeFile::new(dir.join("index.html"))
        .oneshot(req)
        .await
        .unwrap()
        .into_response()
}

async fn static_assets(
    State(dir): State<PathBuf>,
    Path(file): Path<String>,
    req: Request<Body>,
) -> impl IntoResponse {
    if !file.contains('.') {
        let path = dir.join(&file).with_extension("html");
        ServeFile::new(path)
            .oneshot(req)
            .await
            .unwrap()
            .into_response()
    } else {
        let path = dir.join(&file);
        ServeFile::new(path)
            .oneshot(req)
            .await
            .unwrap()
            .into_response()
    }
}