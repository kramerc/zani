# Deployment Guide

This directory contains Ansible playbooks and scripts to deploy the zani IRC bot to a Podman container managed by systemd at the user level.

## Prerequisites

1. **Ansible**: Install Ansible on your control machine
   ```bash
   # Ubuntu/Debian
   sudo apt install ansible
   
   # Fedora
   sudo dnf install ansible
   
   # pip
   pip install ansible
   ```

2. **Podman**: Ensure Podman is installed on the target system
   ```bash
   # Ubuntu/Debian
   sudo apt install podman
   
   # Fedora
   sudo dnf install podman
   ```

## Quick Deployment

1. **Development environment** (default):
   ```bash
   ./deploy.sh
   # or explicitly:
   ./deploy.sh --dev
   ```

2. **Production environment**:
   ```bash
   # Set registry credentials first
   export GHCR_USERNAME="your-github-username"
   export GHCR_TOKEN="ghp_your-personal-access-token"
   
   # Deploy to production
   ./deploy.sh --prod  # Uses production group in inventory.ini
   ```

## Production Registry Setup

For production deployments, you need to authenticate with GitHub Container Registry (ghcr.io):

1. **Create a Personal Access Token**:
   - Go to GitHub Settings → Developer settings → Personal access tokens
   - Generate a new token with `read:packages` permission
   
2. **Set environment variables**:
   ```bash
   export GHCR_USERNAME="your-github-username"
   export GHCR_TOKEN="ghp_your-personal-access-token"
   ```

3. **Deploy**:
   ```bash
   ./deploy.sh --prod
   ```

The playbook will automatically login to the registry and pull the pre-built image.

## Manual Deployment

1. Install required Ansible collections:
   ```bash
   ansible-galaxy collection install -r requirements.yml
   ```

2. Run the playbook:
   ```bash
   ansible-playbook -i inventory.ini deploy.yml
   ```

## What the Playbook Does

1. **User Management**: Creates the `zani` user if it doesn't exist
2. **Directory Setup**: Creates necessary directories under `/home/zani/zani/`
3. **Code Deployment**: Copies source code to the target system
4. **Container Image**: 
   - **Development**: Builds the container image from source using Podman
   - **Production**: Pulls pre-built image from container registry
5. **Container Creation**: Creates and configures the Podman container
6. **Systemd Integration**: 
   - Enables lingering for the zani user
   - Generates systemd service files
   - Enables and starts the service

## File Structure After Deployment

```
/home/zani/zani/
├── config/
│   └── zani.toml          # Bot configuration (auto-generated)
├── data/
│   └── zani.db            # SQLite database file (created by bot)
├── src/                   # Source code
├── Cargo.toml
├── Dockerfile
└── ...
```

## Management Commands

After deployment, you can manage the bot using systemctl:

```bash
# Check status
systemctl --user status container-zani

# View logs
journalctl --user -u container-zani -f

# Restart the bot
systemctl --user restart container-zani

# Stop the bot
systemctl --user stop container-zani

# Start the bot
systemctl --user start container-zani
```

## Configuration

The bot configuration is automatically generated from Ansible variables and stored in `/home/zani/zani/config/zani.toml`.

### Variable Structure

The deployment uses Ansible's group_vars and host_vars for configuration:

```
group_vars/
├── all.yml              # Global variables
└── production.yml       # Production environment variables

host_vars/
└── localhost.yml        # Host-specific variables

templates/
└── zani.toml.j2         # Configuration template
```

### Customizing Configuration

1. **IRC Settings**: Edit `group_vars/all.yml` to modify IRC connection settings:
   ```yaml
   irc_config:
     server: "irc.example.com"
     port: 6697
     use_tls: true
     channels: ["#mychannel"]
   ```

2. **Container Settings**: Modify container runtime options:
   ```yaml
   container_options:
     restart_policy: unless-stopped
     network: bridge
   ```

3. **Host-specific overrides**: Use `host_vars/localhost.yml` for localhost-specific settings

After making changes to variables:

1. Restart the container:
   ```bash
   systemctl --user restart container-zani
   ```

## Troubleshooting

1. **Check container status**:
   ```bash
   podman ps -a
   ```

2. **Check container logs directly**:
   ```bash
   podman logs zani
   ```

3. **Rebuild and redeploy**:
   ```bash
   ./deploy.sh
   ```

4. **Check systemd service status**:
   ```bash
   systemctl --user status container-zani
   ```

## Customization

- **Target Host**: Edit `inventory.ini` to deploy to remote hosts
- **IRC Configuration**: Modify `group_vars/all.yml` or `group_vars/production.yml`
- **Host-specific Settings**: Use `host_vars/localhost.yml` for localhost overrides
- **Container Settings**: Adjust container options in group_vars files
- **Build Options**: Modify build settings and exclude patterns in variables

### Example Customizations

1. **Deploy to development (default)**:
   ```bash
   ./deploy.sh --dev
   # or just
   ./deploy.sh
   ```

2. **Deploy to production**:
   ```bash
   # First, configure your production servers in inventory.ini under [production]
   vim inventory.ini
   
   # Then deploy
   ./deploy.sh --prod
   ```

2. **Use different IRC server**:
   ```yaml
   # group_vars/production.yml
   irc_config:
     server: "irc.libera.chat"
     port: 6697
     use_tls: true
     channels: ["#myproject"]
   ```

3. **Custom container network**:
   ```yaml
   # group_vars/all.yml
   container_options:
     network: bridge
     restart_policy: unless-stopped
   ```
