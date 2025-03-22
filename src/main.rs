use std::collections::HashMap;
use futures::prelude::*;
use irc::client::prelude::*;
use rusqlite::Connection;

struct User {
    id: Option<i64>,
    hostmask: String,
    level: i64,
    auto_op: bool,
    auto_voice: bool,
}

const USER: i64 = 0;
const OP: i64 = 1;
const ADMIN: i64 = 3;

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
        if let Command::PRIVMSG(ref target, ref msg) = message.command {
            let hostmask = &message.prefix.unwrap().to_string();
            let user = get_user(&conn, &hostmask).unwrap_or(default_user(&hostmask));

            if user.level >= OP {
                let command = msg.split(" ").collect::<Vec<&str>>()[0];
                let args = msg.split(" ").collect::<Vec<&str>>()[1..].join(" ");

                match command {
                    "!op" => {
                        let channel = target.to_string();
                        let nick = args;

                        if nick.is_empty() {
                            client.send_privmsg(channel, "Usage: !op <nick>")?;
                            continue;
                        }

                        client.send_mode(channel, &[Mode::Plus(ChannelMode::Oper, Some(nick))])?;
                    }
                    "!addop" => {
                        let channel = target.to_string();
                        let nick = args;

                        if nick.is_empty() {
                            client.send_privmsg(target.to_string(), "Usage: !addop <nick>")?;
                        } else {
                            let hostmask = hostmasks_by_user.get(&nick);
                            if hostmask.is_none() {
                                client.send_privmsg(target.to_string(), format!("The bot is missing the hostmask for user {}", nick))?;
                                continue;
                            }

                            let mut user = get_user(&conn, &hostmask.unwrap())?;
                            user.level = OP;
                            user.auto_op = true;

                            conn.execute(
                                "INSERT OR REPLACE INTO users (hostmask, level, autoOp, autoVoice) VALUES (?1, ?2, ?3, ?4)",
                                (&user.hostmask, &user.level, &user.auto_op, &user.auto_voice),
                            )?;
                            client.send_mode(&channel, &[Mode::Plus(ChannelMode::Oper, Some(nick.clone()))])?;
                            client.send_privmsg(&channel, format!("Added {} to the op list", nick))?;
                        }
                    }
                    "!voice" => {
                        let channel = target.to_string();
                        let nick = args;

                        if nick.is_empty() {
                            client.send_privmsg(channel, "Usage: !voice <nick>")?;
                            continue;
                        }

                        client.send_mode(channel, &[Mode::Plus(ChannelMode::Voice, Some(nick))])?;
                    }
                    "!addvoice" => {
                        let channel = target.to_string();
                        let nick = args;

                        if nick.is_empty() {
                            client.send_privmsg(target.to_string(), "Usage: !addvoice <nick>")?;
                        } else {
                            let hostmask = hostmasks_by_user.get(&nick);
                            if hostmask.is_none() {
                                client.send_privmsg(target.to_string(), format!("The bot is missing the hostmask for user {}", nick))?;
                                continue;
                            }

                            let mut user = get_user(&conn, &hostmask.unwrap())?;
                            user.auto_voice = true;

                            conn.execute(
                                "INSERT OR REPLACE INTO users (hostmask, level, autoOp, autoVoice) VALUES (?1, ?2, ?3, ?4)",
                                (&user.hostmask, &user.level, &user.auto_op, &user.auto_voice),
                            )?;
                            client.send_mode(&channel, &[Mode::Plus(ChannelMode::Oper, Some(nick.clone()))])?;
                            client.send_privmsg(&channel, format!("Added {} to the voice list", nick))?;
                        }
                    }
                    "!who" => {
                        let nick = args;

                        if nick.is_empty() {
                            client.send_privmsg(target.to_string(), "Usage: !who <nick>")?;
                        } else {
                            let hostmask = hostmasks_by_user.get(&nick);
                            if hostmask.is_none() {
                                client.send_privmsg(target.to_string(), format!("The bot is missing the hostmask for user {}", nick))?;
                                continue;
                            }

                            let user = get_user(&conn, &hostmask.unwrap())?;
                            if user.id.is_none() {
                                client.send_privmsg(target.to_string(), format!("{} is not in the database", nick))?
                            } else {
                                client.send_privmsg(target.to_string(), format!("{} is {} and has auto-op: {} and has auto-voice: {}", nick, level_str(user.level), user.auto_op, user.auto_voice))?;
                            }

                            client.send_privmsg(target.to_string(), format!("Hostmask: {}", hostmask.unwrap()))?;
                        }
                    }
                    _ => {}
                }
            }
        } else if let Command::JOIN(ref channel, ..) = message.command {
            let hostmask = &message.prefix.unwrap().to_string();
            let user = get_user(&conn, &hostmask).unwrap_or(default_user(&hostmask));
            let nick = parse_nick(&hostmask);

            hostmasks_by_user.insert(nick.clone(), filtered_hostmask(&hostmask));

            if nick.eq(client.current_nickname()) {
                client.send(Command::WHO(Some(channel.clone()), None))?;
            }

            if user.auto_op {
                let channel = channel.to_string();
                client.send_mode(channel, &[Mode::Plus(ChannelMode::Oper, Some(nick.clone()))])?;
            }

            if user.auto_voice {
                let channel = channel.to_string();
                client.send_mode(channel, &[Mode::Plus(ChannelMode::Voice, Some(nick.clone()))])?;
            }
        } else if let Command::PART(ref channel, ..) = message.command {
            let nick = parse_nick(&message.prefix.unwrap().to_string());
            hostmasks_by_user.remove(&nick);
        } else if let Command::Response(Response::RPL_WHOREPLY, ref args) = message.command {
            // Take data from who reply and store into users vec
            let nick = args[5].to_string();
            let username = args[2].to_string();
            let hostname = args[3].to_string();
            let hostmask = format!("{}!{}@{}", nick, username, hostname);
            hostmasks_by_user.insert(nick, filtered_hostmask(&hostmask));
        }
    }

    Ok(())
}

fn prepare(conn: &Connection) -> rusqlite::Result<usize> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
              id INTEGER PRIMARY KEY,
              hostmask VARCHAR(512) UNIQUE,
              level INT(11),
              autoOp INT(1) DEFAULT 0,
              autoVoice INT(1) DEFAULT 0
            )",
        ()
    )
}

fn seed(conn: &Connection) -> Result<(), anyhow::Error> {
    let users = vec![
        User {
            id: None,
            hostmask: "whale@46.23.92.36".to_string(),
            level: OP,
            auto_op: true,
            auto_voice: false,
        },
        User {
            id: None,
            hostmask: "horse@46.23.95.46".to_string(),
            level: OP,
            auto_op: true,
            auto_voice: false,
        },
        User {
            id: None,
            hostmask: "no@static.226.87.78.5.clients.your-server.de".to_string(),
            level:OP,
            auto_op: true,
            auto_voice: false,
        },
        User {
            id: None,
            hostmask: "kr@m3r.sh".to_string(),
            level: OP,
            auto_op: true,
            auto_voice: false,
        },
        User {
            id: None,
            hostmask: "dolphin@nexus.mer.st".to_string(),
            level: USER,
            auto_op: false,
            auto_voice: true,
        },
        User {
            id: None,
            hostmask: "thelounge@baker.kekra.net".to_string(),
            level: OP,
            auto_op: true,
            auto_voice: false,
        },
    ];

    for user in users {
        conn.execute(
            "INSERT OR IGNORE INTO users (hostmask, level, autoOp, autoVoice) VALUES (?1, ?2, ?3, ?4)",
            (&user.hostmask, &user.level, &user.auto_op, &user.auto_voice),
        )?;
    }

    Ok(())
}

fn get_user(conn: &Connection, hostmask: &str) -> Result<User, anyhow::Error> {
    let user = conn.query_row(
        "SELECT * FROM users WHERE hostmask = ?1",
        [filtered_hostmask(&hostmask)],
        |row| {
            Ok(User {
                id: row.get(0)?,
                hostmask: row.get(1)?,
                level: row.get(2)?,
                auto_op: row.get(3)?,
                auto_voice: row.get(4)?,
            })
        },
    ).unwrap_or(default_user(&hostmask));

    Ok(user)
}

fn default_user(hostmask: &str) -> User {
    User {
        id: None,
        hostmask: hostmask.to_string(),
        level: USER,
        auto_op: false,
        auto_voice: false,
    }
}

fn filtered_hostmask(hostmask: &str) -> String {
    if !hostmask.contains("!") {
        return hostmask.to_string();
    }

    hostmask.replace("~", "").split("!").collect::<Vec<&str>>()[1].to_lowercase().to_string()
}

fn parse_nick(hostmask: &str) -> String {
    hostmask.split("!").collect::<Vec<&str>>()[0].to_string()
}

fn level_str(level: i64) -> String {
    match level {
        0 => "user".to_string(),
        1 => "op".to_string(),
        3 => "admin".to_string(),
        _ => "unknown".to_string(),
    }
}