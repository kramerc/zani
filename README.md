# Sexo IRC Bot

A modular IRC bot written in Rust that provides user management and channel moderation features.

## Project Structure

The codebase has been refactored into a clean modular structure:

- `src/main.rs` - Main application entry point and event loop
- `src/models.rs` - Data models (User struct, constants, utility functions)
- `src/database.rs` - Database operations (SQLite integration)
- `src/commands.rs` - Command handlers (!op, !addop, !voice, !addvoice, !who)
- `src/handlers.rs` - Event handlers (PRIVMSG, JOIN, PART, WHO replies)
- `src/utils.rs` - Utility functions (hostmask parsing, nick extraction)

## Features

- **User Management**: Track users with different privilege levels (USER, OP, ADMIN)
- **Auto-Op/Voice**: Automatically grant privileges when users join
- **Commands**:
  - `!op <nick>` - Grant operator status to a user
  - `!addop <nick>` - Add user to auto-op list
  - `!voice <nick>` - Grant voice to a user  
  - `!addvoice <nick>` - Add user to auto-voice list
  - `!who <nick>` - Display user information and privileges

## Configuration

Create a `sexo.toml` file in the working directory:

```toml
nickname = "sexo"
username = "sexo"
realname = "Sexo IRC Bot"
server = "irc.example.org"
port = 6667
channels = ["#yourchannel"]
```

## Building and Running

```bash
cargo build --release
cargo run
```
