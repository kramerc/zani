use std::collections::HashMap;
use futures::prelude::*;
use irc::client::prelude::*;
use rusqlite::Connection;

mod models;
mod database;
mod commands;
mod handlers;
mod utils;

use database::{prepare, seed};
use handlers::*;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let mut hostmasks_by_user: HashMap<String, String> = HashMap::new();
    let mut channel_modes: ChannelModes = HashMap::new();

    let conn = Connection::open("data/sexo.db")?;
    prepare(&conn)?;
    seed(&conn)?;

    let config = Config::load("config/sexo.toml")?;
    let mut client = Client::from_config(config).await?;
    client.identify()?;

    let mut stream = client.stream()?;

    while let Some(message) = stream.next().await.transpose()? {
        match message.command {
            Command::PRIVMSG(ref target, ref msg) => {
                if let Some(ref prefix) = message.prefix {
                    let hostmask = &prefix.to_string();
                    handle_privmsg(&client, &conn, target, msg, hostmask, &hostmasks_by_user).await?;
                }
            }
            Command::JOIN(ref channel, ..) => {
                if let Some(ref prefix) = message.prefix {
                    let hostmask = &prefix.to_string();
                    handle_join(&client, &conn, channel, hostmask, &mut hostmasks_by_user, &mut channel_modes).await?;
                }
            }
            Command::PART(ref channel, ..) => {
                if let Some(ref prefix) = message.prefix {
                    let hostmask = &prefix.to_string();
                    handle_part(hostmask, &mut hostmasks_by_user, &mut channel_modes, channel);
                }
            }
            Command::Response(Response::RPL_WHOREPLY, ref args) => {
                handle_who_reply(args, &mut hostmasks_by_user);
            }
            Command::Response(Response::RPL_NAMREPLY, ref args) => {
                // args[2] is the channel, args[3] is the list of names
                if args.len() >= 4 {
                    let channel = &args[2];
                    let names_str = &args[3];
                    let names: Vec<String> = names_str.split_whitespace().map(|s| s.to_string()).collect();
                    handle_names_reply(channel, &names, &mut channel_modes);
                }
            }
            Command::Raw(ref cmd, ref args) if cmd == "MODE" => {
                // Handle raw MODE messages
                if args.len() >= 3 {
                    let channel = &args[0];
                    let mode_str = &args[1];
                    handle_raw_mode_change(channel, mode_str, &args[2..], &mut channel_modes);
                }
            }
            _ => {}
        }
    }

    Ok(())
}
