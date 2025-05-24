use crate::models::{User, OP, USER};
use crate::utils::filtered_hostmask;
use rusqlite::Connection;

pub fn prepare(conn: &Connection) -> rusqlite::Result<usize> {
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

pub fn seed(conn: &Connection) -> Result<(), anyhow::Error> {
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
            level: OP,
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

pub fn get_user(conn: &Connection, hostmask: &str) -> Result<User, anyhow::Error> {
    let user = conn.query_row(
        "SELECT * FROM users WHERE hostmask = ?1",
        [filtered_hostmask(&hostmask)],
        |row| User::from_row(row),
    ).unwrap_or(default_user(&hostmask));

    Ok(user)
}

pub fn save_user(conn: &Connection, user: &User) -> Result<(), anyhow::Error> {
    conn.execute(
        "INSERT OR REPLACE INTO users (hostmask, level, autoOp, autoVoice) VALUES (?1, ?2, ?3, ?4)",
        (&user.hostmask, &user.level, &user.auto_op, &user.auto_voice),
    )?;
    Ok(())
}

pub fn default_user(hostmask: &str) -> User {
    User {
        id: None,
        hostmask: hostmask.to_string(),
        level: USER,
        auto_op: false,
        auto_voice: false,
    }
}
