use std::collections::HashMap;
use std::collections::HashSet;
use irc::client::prelude::*;
use rusqlite::Connection;
use crate::models::OP;
use crate::database::get_user;
use crate::utils::{parse_nick, filtered_hostmask, extract_urls, fetch_page_title};
use crate::commands::*;

// Structure to track user modes per channel
#[derive(Debug, Default)]
pub struct ChannelState {
    pub ops: HashSet<String>,
    pub voices: HashSet<String>,
}

pub type ChannelModes = HashMap<String, ChannelState>;

pub async fn handle_privmsg(
    client: &Client,
    conn: &Connection,
    target: &str,
    msg: &str,
    hostmask: &str,
    hostmasks_by_user: &HashMap<String, String>,
) -> Result<(), anyhow::Error> {
    // First, check for URLs in any message and fetch titles
    let urls = extract_urls(msg);
    for url in urls {
        if let Some(title) = fetch_page_title(&url).await {
            let response = format!("\x0303\u{2937}\x03 {}", title);
            client.send_privmsg(target, &response)?;
        }
    }

    // Then handle commands (only for OPs)
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
    channel_modes: &mut ChannelModes,
) -> Result<(), anyhow::Error> {
    let user = get_user(conn, &hostmask)?;
    let nick = parse_nick(&hostmask);

    hostmasks_by_user.insert(nick.clone(), filtered_hostmask(&hostmask));

    if nick.eq(client.current_nickname()) {
        client.send(Command::WHO(Some(channel.to_string()), None))?;
    }

    // Ensure channel state exists
    let channel_state = channel_modes.entry(channel.to_string()).or_default();

    // Check if user should get auto-op and doesn't already have it
    if user.auto_op && !channel_state.ops.contains(&nick) {
        client.send_mode(channel, &[Mode::Plus(ChannelMode::Oper, Some(nick.clone()))])?;
        channel_state.ops.insert(nick.clone());
    }

    // Check if user should get auto-voice and doesn't already have it
    if user.auto_voice && !channel_state.voices.contains(&nick) {
        client.send_mode(channel, &[Mode::Plus(ChannelMode::Voice, Some(nick.clone()))])?;
        channel_state.voices.insert(nick.clone());
    }

    Ok(())
}

pub fn handle_part(
    hostmask: &str,
    hostmasks_by_user: &mut HashMap<String, String>,
    channel_modes: &mut ChannelModes,
    channel: &str,
) {
    let nick = parse_nick(&hostmask);
    hostmasks_by_user.remove(&nick);

    // Remove user from all mode lists for this channel
    if let Some(channel_state) = channel_modes.get_mut(channel) {
        channel_state.ops.remove(&nick);
        channel_state.voices.remove(&nick);
    }
}

pub fn handle_quit(
    hostmask: &str,
    hostmasks_by_user: &mut HashMap<String, String>,
    channel_modes: &mut ChannelModes,
) {
    let nick = parse_nick(&hostmask);
    hostmasks_by_user.remove(&nick);

    // Remove user from all mode lists across all channels
    for channel_state in channel_modes.values_mut() {
        channel_state.ops.remove(&nick);
        channel_state.voices.remove(&nick);
    }
}

pub fn handle_raw_mode_change(
    channel: &str,
    mode_str: &str,
    nicks: &[String],
    channel_modes: &mut ChannelModes,
) {
    let channel_state = channel_modes.entry(channel.to_string()).or_default();

    let mut adding = true;
    let mut nick_index = 0;

    for ch in mode_str.chars() {
        match ch {
            '+' => adding = true,
            '-' => adding = false,
            'o' => {
                if nick_index < nicks.len() {
                    let nick = &nicks[nick_index];
                    if adding {
                        channel_state.ops.insert(nick.clone());
                    } else {
                        channel_state.ops.remove(nick);
                    }
                    nick_index += 1;
                }
            }
            'v' => {
                if nick_index < nicks.len() {
                    let nick = &nicks[nick_index];
                    if adding {
                        channel_state.voices.insert(nick.clone());
                    } else {
                        channel_state.voices.remove(nick);
                    }
                    nick_index += 1;
                }
            }
            _ => {} // Ignore other modes
        }
    }
}

pub fn handle_names_reply(
    channel: &str,
    names: &[String],
    channel_modes: &mut ChannelModes,
) {
    let channel_state = channel_modes.entry(channel.to_string()).or_default();

    for name in names {
        if name.starts_with('@') {
            let nick = &name[1..];
            channel_state.ops.insert(nick.to_string());
            // Users with @ might also have +, but @ includes voice privileges
        } else if name.starts_with('+') {
            let nick = &name[1..];
            channel_state.voices.insert(nick.to_string());
        }
        // Regular users have no prefix
    }
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
