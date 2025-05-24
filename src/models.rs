use rusqlite::Row;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Option<i64>,
    pub hostmask: String,
    pub level: i64,
    pub auto_op: bool,
    pub auto_voice: bool,
}

impl User {
    pub fn from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(User {
            id: row.get(0)?,
            hostmask: row.get(1)?,
            level: row.get(2)?,
            auto_op: row.get(3)?,
            auto_voice: row.get(4)?,
        })
    }
}

pub const USER: i64 = 0;
pub const OP: i64 = 1;
pub const ADMIN: i64 = 3;

pub fn level_str(level: i64) -> String {
    match level {
        0 => "user".to_string(),
        1 => "op".to_string(),
        3 => "admin".to_string(),
        _ => "unknown".to_string(),
    }
}
