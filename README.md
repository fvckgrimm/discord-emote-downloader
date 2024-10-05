# Discord Emote Downloader

Discord Emote Downloader is a command-line tool that allows you to download emotes and stickers from Discord servers (guilds) you're a member of.

## TODO:

- [ ] Switch to ratatui for interactive mode

## Features

- Download emotes and stickers from specific Discord guilds
- Dump guild information into a JSON file
- Interactive mode to select guilds from a list
- Option to download from all guilds at once

## Installation

1. Make sure you have Rust installed on your system. If not, you can install it from [https://www.rust-lang.org/](https://www.rust-lang.org/).

2. Clone this repository:

```bash 
git clone https://github.com/fvckgrimm/discord-emote-downloader && cd discord-emote-downloader
```

3. Build the project: 

```bash
cargo build --release
```

4. The executable will be available in `target/release/discord-emote-downloader`.

## Usage

Before using the tool, you need to set up your Discord token. Rename the `./example-settings.json` file to `settings.json` in the same directory you'll be running the executable from and replace it with your token.

### Basic Usage

Run the tool without any arguments to enter interactive mode:

```bash
./discord-emote-downloader
```

### Command-line Options

* -t, --token <TOKEN>: Use specified token instead of loading from settings
* -d, --dir <DIR>: Directory where files should be created
* -g, --guild <GUILD_ID>: Dump emotes from specified guild
* -j, --json: Dump guild info into a JSON file instead of creating an archive

### Examples

1. Download emotes and stickers from a specific guild:

```bash
./discord-emote-downloader -g 1234567890
```

2. Dump guild information to a JSON file:

```bash
./discord-emote-downloader -g 1234567890 -j
```

3. Use a custom token and output directory:

```bash
./discord-emote-downloader -t your_token_here -d /path/to/output
```

## Output

* Emotes and stickers are saved in a ZIP archive named Emotes_Stickers_<GuildName>.zip in the ./emotes directory.
* JSON dumps (when using the -j flag) are saved as <GuildName>.json in the ./emotes directory.

## Logging

You can control the log level by setting the RUST_LOG environment variable. For example:

```bash
RUST_LOG=debug ./discord-emote-downloader
```
