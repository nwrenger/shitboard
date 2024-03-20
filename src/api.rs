use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use std::{
    path::PathBuf,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::error::{Error, Result};
use axum::{extract::State, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use tokio::{fs, io::AsyncReadExt};

#[derive(Debug, Serialize, Deserialize)]

struct Resource {
    title: String,
    audio_file: PathBuf,
    picture_file: PathBuf,
    time_stamp: u64,
}

#[derive(Debug, Deserialize)]
struct Files {
    title: String,
    audio_data: String,
    picture_data: String,
}

#[derive(Clone)]
pub struct Project {
    resource_path: Arc<PathBuf>,
}

impl Project {
    pub fn new(resource_path: PathBuf) -> Self {
        Self {
            resource_path: Arc::new(resource_path),
        }
    }
}

pub fn routes(state: Project) -> Router {
    Router::new()
        .route("/resource", get(resources).post(add_resource))
        .with_state(state)
}

async fn resources(State(project): State<Project>) -> Result<Json<Vec<Resource>>> {
    let mut resources = Vec::new();

    let mut entries = fs::read_dir(project.resource_path.to_path_buf()).await?;

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.extension().and_then(std::ffi::OsStr::to_str) == Some("json") {
            let mut file = fs::File::open(&path).await?;
            let mut contents = String::new();
            file.read_to_string(&mut contents).await?;

            let resource: Resource = serde_json::from_str(&contents)?;
            resources.push(resource);
        }
    }

    Ok(Json(resources))
}

async fn add_resource(
    State(project): State<Project>,
    Json(files): Json<Files>,
) -> Result<Json<Resource>> {
    let resource_path = project.resource_path.to_string_lossy();
    let audio_data = BASE64_STANDARD.decode(files.audio_data)?;
    let picture_data = BASE64_STANDARD.decode(files.picture_data)?;

    let (audio_ext, audio_mime) = infer_ext(&audio_data).unwrap_or_default();
    let (picture_ext, picture_mime) = infer_ext(&picture_data).unwrap_or_default();

    if !audio_mime.starts_with("audio/") || !picture_mime.starts_with("image/") {
        return Err(Error::InvalidFileType);
    }

    let audio_file = format!("{}{}.{}", resource_path, files.title, audio_ext);
    let picture_file = format!("{}{}.{}", resource_path, files.title, picture_ext);
    let json_file = format!("{}/{}.json", resource_path, files.title);

    if !fs::try_exists(&json_file).await? {
        let resource = Resource {
            title: files.title.clone(),
            audio_file: audio_file.clone().into(),
            picture_file: picture_file.clone().into(),
            time_stamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        };

        fs::write(&audio_file, audio_data).await?;
        fs::write(&picture_file, picture_data).await?;
        fs::write(&json_file, serde_json::to_string(&resource)?).await?;

        Ok(Json(resource))
    } else {
        Err(Error::FileAlreadyExists)
    }
}

fn infer_ext(data: &[u8]) -> Option<(String, String)> {
    infer::get(data).map(|info| (info.extension().to_string(), info.mime_type().to_string()))
}
