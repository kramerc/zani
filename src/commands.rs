use std::collections::HashMap;
use irc::client::prelude::*;
use rusqlite::Connection;
use crate::models::{level_str, OP};
use crate::database::{get_user, save_user};

pub async fn handle_op_command(
    client: &Client,
    channel: &str,
    nick: &str,
) -> Result<(), anyhow::Error> {
    if nick.is_empty() {
        client.send_privmsg(channel, "Usage: !op <nick>")?;
        return Ok(());
    }

    client.send_mode(channel, &[Mode::Plus(ChannelMode::Oper, Some(nick.to_string()))])?;
    Ok(())
}

pub async fn handle_addop_command(
    client: &Client,
    conn: &Connection,
    channel: &str,
    nick: &str,
    hostmasks_by_user: &HashMap<String, String>,
) -> Result<(), anyhow::Error> {
    if nick.is_empty() {
        client.send_privmsg(channel, "Usage: !addop <nick>")?;
        return Ok(());
    }

    let hostmask = hostmasks_by_user.get(nick);
    if hostmask.is_none() {
        client.send_privmsg(channel, format!("The bot is missing the hostmask for user {}", nick))?;
        return Ok(());
    }

    let mut user = get_user(conn, &hostmask.unwrap())?;
    user.level = OP;
    user.auto_op = true;

    save_user(conn, &user)?;
    client.send_mode(channel, &[Mode::Plus(ChannelMode::Oper, Some(nick.to_string()))])?;
    client.send_privmsg(channel, format!("Added {} to the op list", nick))?;

    Ok(())
}

pub async fn handle_voice_command(
    client: &Client,
    channel: &str,
    nick: &str,
) -> Result<(), anyhow::Error> {
    if nick.is_empty() {
        client.send_privmsg(channel, "Usage: !voice <nick>")?;
        return Ok(());
    }

    client.send_mode(channel, &[Mode::Plus(ChannelMode::Voice, Some(nick.to_string()))])?;
    Ok(())
}

pub async fn handle_addvoice_command(
    client: &Client,
    conn: &Connection,
    channel: &str,
    nick: &str,
    hostmasks_by_user: &HashMap<String, String>,
) -> Result<(), anyhow::Error> {
    if nick.is_empty() {
        client.send_privmsg(channel, "Usage: !addvoice <nick>")?;
        return Ok(());
    }

    let hostmask = hostmasks_by_user.get(nick);
    if hostmask.is_none() {
        client.send_privmsg(channel, format!("The bot is missing the hostmask for user {}", nick))?;
        return Ok(());
    }

    let mut user = get_user(conn, &hostmask.unwrap())?;
    user.auto_voice = true;

    save_user(conn, &user)?;
    client.send_mode(channel, &[Mode::Plus(ChannelMode::Voice, Some(nick.to_string()))])?;
    client.send_privmsg(channel, format!("Added {} to the voice list", nick))?;

    Ok(())
}

pub async fn handle_who_command(
    client: &Client,
    conn: &Connection,
    channel: &str,
    nick: &str,
    hostmasks_by_user: &HashMap<String, String>,
) -> Result<(), anyhow::Error> {
    if nick.is_empty() {
        client.send_privmsg(channel, "Usage: !who <nick>")?;
        return Ok(());
    }

    let hostmask = hostmasks_by_user.get(nick);
    if hostmask.is_none() {
        client.send_privmsg(channel, format!("The bot is missing the hostmask for user {}", nick))?;
        return Ok(());
    }

    let user = get_user(conn, &hostmask.unwrap())?;
    if user.id.is_none() {
        client.send_privmsg(channel, format!("{} is not in the database", nick))?;
    } else {
        client.send_privmsg(
            channel,
            format!(
                "{} is {} and has auto-op: {} and has auto-voice: {}",
                nick,
                level_str(user.level),
                user.auto_op,
                user.auto_voice
            ),
        )?;
    }

    client.send_privmsg(channel, format!("Hostmask: {}", hostmask.unwrap()))?;
    Ok(())
}
