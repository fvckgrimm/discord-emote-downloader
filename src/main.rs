mod archive;
mod downlaod;
mod guilds;
mod utils;

use crate::guilds::{dump_emotes_and_stickers, load_guilds, print_guilds, Guild};
use crate::utils::load_token;
use log::info;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use std::error::Error;
use std::io::{self, Write};
use structopt::StructOpt;
use tokio;

const DISCORD_API: &str = "https://discord.com/api/v8/";
const DISCORD_CDN: &str = "https://cdn.discordapp.com/";

#[derive(Debug, StructOpt)]
#[structopt(
    name = "discord_emote_downloader",
    about = "Download Discord emotes and stickers"
)]
struct Opt {
    #[structopt(
        short,
        long,
        help = "Use specified token instead of loading from settings"
    )]
    token: Option<String>,

    #[structopt(short, long, help = "Directory where files should be created")]
    dir: Option<String>,

    #[structopt(short, long, help = "Dump emotes from specified guild")]
    guild: Option<String>,

    #[structopt(
        short,
        long,
        help = "Dump guild info into a JSON file instead of creating an archive"
    )]
    json: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let opt = Opt::from_args();
    let token = opt.token.unwrap_or_else(load_token);

    if let Some(dir) = opt.dir {
        std::env::set_current_dir(dir)?;
    }

    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert("Authorization", HeaderValue::from_str(&token)?);
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static(
            "Mozilla/5.0 (X11; Linux x86_64; rv:130.0) Gecko/20100101 Firefox/130.0",
        ),
    );

    if let Some(guild_id) = opt.guild {
        dump_emotes_and_stickers(&client, &headers, &guild_id, opt.json).await?;
    } else {
        let guilds = load_guilds(&client, &headers).await?;
        main_loop(&client, &headers, &guilds).await?;
    }

    Ok(())
}

async fn main_loop(
    client: &reqwest::Client,
    headers: &HeaderMap,
    guilds: &[Guild],
) -> Result<(), Box<dyn Error>> {
    loop {
        print_guilds(guilds);
        println!(
            "\n[A] Dump emotes and stickers from all guilds\n[R] Print guild list\n[Q] Quit\n"
        );
        print!("Guild Index > ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim().to_lowercase();

        match input.as_str() {
            "a" => {
                info!("Dumping from all guilds... (This may take a while)");
                for guild in guilds {
                    dump_emotes_and_stickers(client, headers, &guild.id, false).await?;
                }
            }
            "r" => continue,
            "q" => {
                println!("Goodbye! (^_^)／");
                break;
            }
            _ => {
                if let Ok(index) = input.parse::<usize>() {
                    if index > 0 && index <= guilds.len() {
                        dump_emotes_and_stickers(client, headers, &guilds[index - 1].id, false)
                            .await?;
                    }
                }
            }
        }
    }
    Ok(())
}
