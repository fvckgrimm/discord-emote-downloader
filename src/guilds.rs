use crate::archive::create_archive;
use crate::downlaod::{download_emote, download_sticker};
use crate::downlaod::{Emote, Sticker};
use crate::utils::sanitize_filename;
use crate::DISCORD_API;
use log::{debug, error, info, warn};
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{self};

#[derive(Serialize, Deserialize, Debug)]
pub struct Guild {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub owner: bool,
    pub permissions: String,
    pub features: Vec<String>,
}

pub async fn load_guilds(
    client: &reqwest::Client,
    headers: &HeaderMap,
) -> Result<Vec<Guild>, Box<dyn Error>> {
    let url = format!("{}{}", DISCORD_API, "users/@me/guilds");
    let response = client.get(&url).headers(headers.clone()).send().await?;

    let status = response.status();
    let text = response.text().await?;
    debug!("Raw API response: {}", text);

    if !status.is_success() {
        error!("API request failed with status code: {}", status);
        error!("Response content: {}", text);
        return Err(format!("API request failed: {} - {}", status, text).into());
    }

    let guilds: Vec<Guild> = match serde_json::from_str(&text) {
        Ok(guilds) => guilds,
        Err(e) => {
            error!("Failed to parse API response: {}", e);
            error!("Response content: {}", text);
            return Err(Box::new(e));
        }
    };

    if guilds.is_empty() {
        warn!("No guilds found in the API response");
    } else {
        info!("Successfully loaded {} guilds", guilds.len());
    }

    Ok(guilds)
}

pub fn print_guilds(guilds: &[Guild]) {
    println!("{}", "-".repeat(50));
    for (index, guild) in guilds.iter().enumerate() {
        println!("[{:02}] {}", index + 1, guild.name);
    }
    println!("{}", "-".repeat(50));
}

pub async fn dump_emotes_and_stickers(
    client: &reqwest::Client,
    headers: &HeaderMap,
    guild_id: &str,
    json_dump: bool,
) -> Result<(), Box<dyn Error>> {
    let url = format!("{}guilds/{}", DISCORD_API, guild_id);
    let response = client.get(&url).headers(headers.clone()).send().await?;

    if response.status() != reqwest::StatusCode::OK {
        error!("Failed to dump guild emotes and stickers, unknown guild");
        return Err("Failed to dump guild emotes and stickers, unknown guild".into());
    }

    let guild_data: serde_json::Value = response.json().await?;
    let guild_name = sanitize_filename(&guild_data["name"].as_str().unwrap());

    if json_dump {
        let json_path = format!("./emotes/{}.json", guild_name);
        fs::write(&json_path, serde_json::to_string_pretty(&guild_data)?)?;
        info!("Dumped guild info into {}", json_path);
    } else {
        let emotes: Vec<Emote> = serde_json::from_value(guild_data["emojis"].clone())?;
        let stickers: Vec<Sticker> = guild_data["stickers"]
            .as_array()
            .map(|arr| {
                serde_json::from_value(serde_json::Value::Array(arr.clone())).unwrap_or_default()
            })
            .unwrap_or_default();

        info!(
            "Dumping {} emotes and {} stickers from {}",
            emotes.len(),
            stickers.len(),
            guild_name
        );

        let mut results = Vec::new();

        for emote in emotes {
            match download_emote(client, &emote).await {
                Ok(data) => results.push(data),
                Err(e) => error!("Failed to download emote {}: {}", emote.name, e),
            }
        }

        for sticker in stickers {
            match download_sticker(client, &sticker).await {
                Ok(data) => results.push(data),
                Err(e) => error!("Failed to download sticker {}: {}", sticker.name, e),
            }
        }

        create_archive(&guild_name, &results)?;
        info!("Done.");
    }

    Ok(())
}
