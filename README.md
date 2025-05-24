# Sexo IRC Bot

A modular IRC bot written in Rust that provides user management and channel moderation features.

## Quick Start

```bash
# Clone and deploy locally
git clone <repository>
cd sexo
./deploy.sh --dev
```

## Project Structure

### Source Code
The codebase has been refactored into a clean modular structure:

- `src/main.rs` - Main application entry point and event loop
- `src/models.rs` - Data models (User struct, constants, utility functions)
- `src/database.rs` - Database operations (SQLite integration)
- `src/commands.rs` - Command handlers (!op, !addop, !voice, !addvoice, !who)
- `src/handlers.rs` - Event handlers (PRIVMSG, JOIN, PART, WHO replies)
- `src/utils.rs` - Utility functions (hostmask parsing, nick extraction)

### Deployment Infrastructure
- `deploy.yml` - Main Ansible playbook for container deployment
- `group_vars/` - Environment-specific configuration variables
- `host_vars/` - Host-specific configuration overrides
- `templates/` - Configuration file templates
- `inventory.ini` - Single Ansible inventory file with development and production groups

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

The bot can be configured in two ways:

### Manual Configuration
Create a `config/sexo.toml` file:

```toml
nickname = "sexo"
username = "sexo"
realname = "Sexo IRC Bot"
server = "irc.example.org"
port = 6667
channels = ["#yourchannel"]
```

### Ansible Deployment (Recommended)
Use the included Ansible playbooks for automated deployment to Podman containers:

```bash
# Development deployment
./deploy.sh --dev

# Production deployment  
./deploy.sh --prod
```

See [DEPLOYMENT.md](DEPLOYMENT.md) for detailed deployment instructions and [VARIABLES.md](VARIABLES.md) for configuration options.

## Building and Running

### Local Development
```bash
cargo build --release
cargo run
```

### Container Deployment
```bash
# Build and run with Docker Compose
docker-compose up --build

# Or use Ansible for production deployment
./deploy.sh
```
