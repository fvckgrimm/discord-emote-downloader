use crate::DISCORD_CDN;
use log::debug;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Emote {
    pub id: String,
    pub name: String,
    pub animated: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sticker {
    pub id: String,
    pub name: String,
    pub format_type: u8,
}

#[derive(Debug)]
pub struct DownloadedItem {
    pub name: String,
    pub extension: String,
    pub data: Vec<u8>,
    pub item_type: ItemType,
}

#[derive(Debug)]
pub enum ItemType {
    Emote,
    Sticker,
}

pub async fn download_emote(
    client: &reqwest::Client,
    emote: &Emote,
) -> Result<DownloadedItem, Box<dyn Error>> {
    let extension = if emote.animated { ".gif" } else { ".png" };
    let route = format!("emojis/{}{}", emote.id, extension);
    let url = format!("{}{}", DISCORD_CDN, route);

    let response = client.get(&url).send().await?;
    if response.status() == reqwest::StatusCode::OK {
        let data = response.bytes().await?;
        Ok(DownloadedItem {
            name: emote.name.clone(),
            extension: extension.to_string(),
            data: data.to_vec(),
            item_type: ItemType::Emote,
        })
    } else {
        debug!("Failed to download emote:id:{} from {}", emote.id, url);
        Err("Failed to download emote".into())
    }
}

pub async fn download_sticker(
    client: &reqwest::Client,
    sticker: &Sticker,
) -> Result<DownloadedItem, Box<dyn Error>> {
    let (extension, url) = match sticker.format_type {
        1 => (
            ".png",
            format!("{}stickers/{}.png", DISCORD_CDN, sticker.id),
        ),
        2 => (
            ".png",
            format!("{}stickers/{}.png", DISCORD_CDN, sticker.id),
        ),
        3 => (
            ".json",
            format!("{}stickers/{}.json", DISCORD_CDN, sticker.id),
        ),
        4 => (
            ".gif",
            format!("https://media.discordapp.net/stickers/{}.gif", sticker.id),
        ),
        _ => (
            ".png",
            format!("{}stickers/{}.png", DISCORD_CDN, sticker.id),
        ),
    };

    let response = client.get(&url).send().await?;

    if response.status() == reqwest::StatusCode::OK {
        let data = response.bytes().await?;
        Ok(DownloadedItem {
            name: sticker.name.clone(),
            extension: extension.to_string(),
            data: data.to_vec(),
            item_type: ItemType::Sticker,
        })
    } else {
        debug!("Failed to download sticker:id:{} from {}", sticker.id, url);
        Err(format!("Failed to download sticker: HTTP {}", response.status()).into())
    }
}
