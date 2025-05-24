use std::collections::HashMap;
use irc::client::prelude::*;
use rusqlite::Connection;
use crate::models::OP;
use crate::database::get_user;
use crate::utils::{parse_nick, filtered_hostmask};
use crate::commands::*;

pub async fn handle_privmsg(
    client: &Client,
    conn: &Connection,
    target: &str,
    msg: &str,
    hostmask: &str,
    hostmasks_by_user: &HashMap<String, String>,
) -> Result<(), anyhow::Error> {
    let user = get_user(conn, &hostmask)?;

    if user.level < OP {
        return Ok(());
    }

    let parts: Vec<&str> = msg.split_whitespace().collect();
    if parts.is_empty() {
        return Ok(());
    }

    let command = parts[0];
    let args = if parts.len() > 1 {
        parts[1..].join(" ")
    } else {
        String::new()
    };

    match command {
        "!op" => handle_op_command(client, target, &args).await?,
        "!addop" => handle_addop_command(client, conn, target, &args, hostmasks_by_user).await?,
        "!voice" => handle_voice_command(client, target, &args).await?,
        "!addvoice" => handle_addvoice_command(client, conn, target, &args, hostmasks_by_user).await?,
        "!who" => handle_who_command(client, conn, target, &args, hostmasks_by_user).await?,
        _ => {}
    }

    Ok(())
}

pub async fn handle_join(
    client: &Client,
    conn: &Connection,
    channel: &str,
    hostmask: &str,
    hostmasks_by_user: &mut HashMap<String, String>,
) -> Result<(), anyhow::Error> {
    let user = get_user(conn, &hostmask)?;
    let nick = parse_nick(&hostmask);

    hostmasks_by_user.insert(nick.clone(), filtered_hostmask(&hostmask));

    if nick.eq(client.current_nickname()) {
        client.send(Command::WHO(Some(channel.to_string()), None))?;
    }

    if user.auto_op {
        client.send_mode(channel, &[Mode::Plus(ChannelMode::Oper, Some(nick.clone()))])?;
    }

    if user.auto_voice {
        client.send_mode(channel, &[Mode::Plus(ChannelMode::Voice, Some(nick.clone()))])?;
    }

    Ok(())
}

pub fn handle_part(
    hostmask: &str,
    hostmasks_by_user: &mut HashMap<String, String>,
) {
    let nick = parse_nick(&hostmask);
    hostmasks_by_user.remove(&nick);
}

pub fn handle_who_reply(
    args: &[String],
    hostmasks_by_user: &mut HashMap<String, String>,
) {
    let nick = args[5].to_string();
    let username = args[2].to_string();
    let hostname = args[3].to_string();
    let hostmask = format!("{}!{}@{}", nick, username, hostname);
    hostmasks_by_user.insert(nick, filtered_hostmask(&hostmask));
}
