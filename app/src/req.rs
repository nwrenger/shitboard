use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::json;

const BASE_URL: &str = "http://127.0.0.1:5000";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]

pub struct Resource {
    pub title: String,
    pub audio_file: PathBuf,
    pub picture_file: PathBuf,
    pub time_stamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Files {
    pub title: String,
    pub audio_data: String,
    pub picture_data: String,
}

/// Getting resources
pub async fn get_resources(client: &reqwest::Client) -> Result<Vec<Resource>, reqwest::Error> {
    let url = format!("{BASE_URL}/api/resource");
    let response = client.get(url).send().await?;

    if response.status().is_success() {
        let json: Vec<Resource> = response.json().await?;
        Ok(json)
    } else {
        Err(response.error_for_status().unwrap_err())
    }
}

/// Add a resource
pub async fn add_resource(
    client: &reqwest::Client,
    data: Files,
) -> Result<Resource, reqwest::Error> {
    let url = format!("{BASE_URL}/api/resource");
    let response = client
        .post(url)
        .header("content-type", "application/json")
        .json(&json!(data))
        .send()
        .await?;
    if response.status().is_success() {
        let json: Resource = response.json().await?;
        Ok(json)
    } else {
        Err(response.error_for_status().unwrap_err())
    }
}

pub async fn download_file(
    client: &reqwest::Client,
    path: &str,
) -> Result<bytes::Bytes, reqwest::Error> {
    let url = format!("{BASE_URL}/{path}");
    let response = client.get(url).send().await?;

    if response.status().is_success() {
        let bytes = response.bytes().await?;
        Ok(bytes)
    } else {
        Err(response.error_for_status().unwrap_err())
    }
}
