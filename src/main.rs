mod config;
mod models;

use std::error::Error;
use std::{env, fs};
use reqwest::Client;
use rand::seq::SliceRandom;
use wallpaper::set_from_path;
use models::{Reply, Photo};

async fn get_reply() -> Result<Reply, Box<dyn Error>> {
    let client = Client::new();
    let response = client
        .get(config::API_URL)
        .header("Authorization", config::API_KEY)
        .send()
        .await?
        .json::<Reply>()
        .await?;
    Ok(response)
}

async fn get_random_photo(reply: Reply) -> Result<Photo, Box<dyn Error>> {
    let mut photos = reply.photos;
    photos.shuffle(&mut rand::thread_rng());
    let photo = photos
        .into_iter()
        .next()
        .unwrap();
    Ok(photo)
}

async fn download_photo(url: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let response = client
        .get(url)
        .send()
        .await?;
    let bytes = response
        .bytes()
        .await?;
    fs::write(config::PHOTO_PATH, bytes)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let reply = get_reply().await?;
    let photo = get_random_photo(reply).await?;
    let url = photo.src.original;
    let mut full_path = env::current_dir()?;
    full_path.push(config::PHOTO_PATH);
    download_photo(&url).await?;
    set_from_path(full_path.to_str().unwrap())?;
    Ok(())
}