// IRC Bot Refactoring - Clean Modular Structure
//
// This file has been refactored from a monolithic structure into a clean modular design:
// - Separated concerns into logical modules (models, database, commands, handlers, utils)
// - Extracted command handling into dedicated functions
// - Improved code readability and maintainability
// - Made the codebase easier to extend with new features

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

    let conn = Connection::open("sexo.db")?;
    prepare(&conn)?;
    seed(&conn)?;

    let config = Config::load("sexo.toml")?;
    let mut client = Client::from_config(config).await?;
    client.identify()?;

    let mut stream = client.stream()?;

    while let Some(message) = stream.next().await.transpose()? {
        match message.command {
            Command::PRIVMSG(ref target, ref msg) => {
                let hostmask = &message.prefix.unwrap().to_string();
                handle_privmsg(&client, &conn, target, msg, hostmask, &hostmasks_by_user).await?;
            }
            Command::JOIN(ref channel, ..) => {
                let hostmask = &message.prefix.unwrap().to_string();
                handle_join(&client, &conn, channel, hostmask, &mut hostmasks_by_user).await?;
            }
            Command::PART(ref _channel, ..) => {
                let hostmask = &message.prefix.unwrap().to_string();
                handle_part(hostmask, &mut hostmasks_by_user);
            }
            Command::Response(Response::RPL_WHOREPLY, ref args) => {
                handle_who_reply(args, &mut hostmasks_by_user);
            }
            _ => {}
        }
    }

    Ok(())
}
