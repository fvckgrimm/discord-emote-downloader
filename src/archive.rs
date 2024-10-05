use crate::downlaod::{DownloadedItem, ItemType};
use crate::utils::ensure_emotes_directory;
use log::{debug, info};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use zip::write::FileOptions;
use zip::ZipWriter;

pub fn create_archive(guild_name: &str, items: &[DownloadedItem]) -> Result<(), Box<dyn Error>> {
    let emotes_dir = ensure_emotes_directory();
    let zip_path = emotes_dir.join(format!("Emotes_Stickers_{}.zip", guild_name));
    info!("Adding emotes and stickers to {:?}", zip_path);

    let file = File::create(&zip_path)?;
    let mut zip = ZipWriter::new(file);
    let options: FileOptions<'_, ()> =
        FileOptions::default().compression_method(zip::CompressionMethod::Stored);

    let mut added_files = Vec::new();

    for item in items {
        let mut filename = format!("{}{}", item.name, item.extension);
        let folder = match item.item_type {
            ItemType::Emote if item.extension == ".gif" => "animated_emotes",
            ItemType::Emote => "static_emotes",
            ItemType::Sticker => "stickers",
        };

        let mut full_path = format!("{}/{}", folder, filename);

        if added_files.contains(&full_path) {
            let count = added_files.iter().filter(|&f| f == &full_path).count();
            filename = format!(
                "{}~{}{}",
                &filename[..filename.len() - item.extension.len()],
                count + 1,
                item.extension
            );
            full_path = format!("{}/{}", folder, filename);
            debug!("Duplicate detected, new filename: {}", filename);
        }

        zip.start_file(full_path.clone(), options)?;
        zip.write_all(&item.data)?;
        added_files.push(full_path);
    }

    zip.finish()?;
    info!("Archive created: {:?}", zip_path);
    Ok(())
}
