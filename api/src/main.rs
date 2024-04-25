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
use tokio::fs;
use tower::util::ServiceExt;
use tower_http::services::ServeFile;
use tracing::info;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Ip and port for the webserver
    host: SocketAddr,
    /// Directory for the static assets
    #[arg(short, long, default_value = "../web/build")]
    assets: PathBuf,
    /// Path to where the files are getting saved
    #[arg(short, long, default_value = "files")]
    resources: PathBuf,
}

#[tokio::main]
async fn main() {
    logging();

    let Args {
        host,
        assets,
        resources,
    } = Args::parse();

    if !fs::try_exists(&assets).await.unwrap_or(false)
        || !fs::try_exists(&resources).await.unwrap_or(false)
    {
        panic!("The configured Path doesn't exists, Please change them!");
    }

    let project = Project::new(resources.clone());

    let resource_route = &format!(
        "/{}*file",
        if resources
            .to_string_lossy()
            .chars()
            .last()
            .unwrap_or_default()
            != '/'
        {
            resources.to_string_lossy() + "/"
        } else {
            resources.to_string_lossy()
        }
    );

    let app = Router::new()
        .nest("/api", api::routes(project))
        .route("/", get(static_index).with_state(assets.clone()))
        .route(resource_route, get(static_assets).with_state(resources))
        .route("/*file", get(static_assets).with_state(assets));

    info!("Starting on {host}");

    let listener = tokio::net::TcpListener::bind(host).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// initialize tracing
fn logging() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();
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
