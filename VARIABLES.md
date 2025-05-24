# Group Variables Reference

This file documents all available variables for the sexo IRC bot deployment.

## Core Application Variables

```yaml
# Application identity
app_name: sexo                    # Name of the application
app_user: sexo                    # System user to run the application
app_dir: "/home/{{ app_user }}/{{ app_name }}"  # Application directory
config_dir: "{{ app_dir }}/config"              # Configuration directory
data_dir: "{{ app_dir }}/data"                  # Persistent data directory

# Container settings
container_name: "{{ app_name }}"                # Container name
container_image: "localhost/{{ app_name }}:latest"  # Container image name
systemd_service: "container-{{ container_name }}"   # Systemd service name
```

## IRC Configuration

```yaml
irc_config:
  nickname: "sexo"              # Primary nickname
  alt_nicks:                    # Alternative nicknames
    - "sexo_"
    - "sexo__"
  username: "sexo"              # IRC username
  realname: "sexo"              # IRC real name
  server: "localhost"           # IRC server hostname
  port: 6667                    # IRC server port
  password: ""                  # Server password (if required)
  use_tls: false               # Use TLS/SSL connection
  encoding: "UTF-8"            # Text encoding
  channels:                    # Channels to join
    - "#thelounge"
  umodes: "+i"                 # User modes to set
  user_info: "sexo channel management bot"  # User info
  version: "sexo/0.1.0"        # Version string
  ping_time: 180               # Ping interval
  ping_timeout: 20             # Ping timeout
  burst_window_length: 8       # Flood protection window
  max_messages_in_burst: 15    # Max messages in burst
  should_ghost: false          # Enable GHOST command
  channel_keys: {}             # Channel passwords/keys (see below)
```

### Channel Keys Configuration

Channel keys (passwords) can be configured per environment:

```yaml
irc_config:
  channel_keys:
    "#private": "channel-password"
    "#secret": "another-key"
```

## Container Runtime Options

```yaml
container_options:
  restart_policy: always       # Container restart policy (always, unless-stopped, no)
  network: host               # Container network mode (host, bridge, none)
  log_driver: journald        # Logging driver (journald, json-file, none)
  log_tag: "{{ container_name }}"  # Log tag for identification
```

## Build Strategy and Options

```yaml
# Build strategy - determines how container images are obtained
build_strategy: "build"        # Options: "build" (from source) or "pull" (from registry)

build_options:
  force_rebuild: true         # Force rebuild/repull of container image
  exclude_patterns:          # Patterns to exclude from source copy (build strategy only)
    - "target/"
    - ".git/"
    - "*.db"
    - "deploy.yml"
    - "inventory.ini"
    - "group_vars/"
    - "host_vars/"
```

### Build Strategy Details

- **`build`**: Builds container image from source code in the deployment directory
  - Used in development environments
  - Requires Dockerfile in the project root
  - Excludes patterns specified in `build_options.exclude_patterns`
  
- **`pull`**: Pulls pre-built container image from a registry
  - Used in production environments
  - Requires `container_image` to point to a registry URL
  - Example: `ghcr.io/kramerc/sexo:main`

### Environment Examples

**Development (`group_vars/development.yml`):**
```yaml
build_strategy: "build"
container_image: "localhost/{{ app_name }}:latest"  # Built locally
```

**Production (`group_vars/production.yml`):**
```yaml
build_strategy: "pull"
container_image: "ghcr.io/kramerc/{{ app_name }}:main"  # Pulled from registry

# Registry authentication using environment variables
registry_auth:
  registry: "ghcr.io"
  username: "{{ lookup('env', 'GHCR_USERNAME') }}"
  password: "{{ lookup('env', 'GHCR_TOKEN') }}"
```

## Registry Authentication

For production deployments that pull from container registries, you need to provide authentication credentials via environment variables:

```bash
# GitHub Container Registry example
export GHCR_USERNAME="your-github-username"
export GHCR_TOKEN="ghp_your-personal-access-token"

# Then deploy
./deploy.sh --prod
```

The registry authentication is automatically used when:
- `build_strategy` is set to `"pull"`
- `registry_auth` is defined with valid credentials
- Environment variables `GHCR_USERNAME` and `GHCR_TOKEN` are set

**Note**: For GitHub Container Registry (ghcr.io), use a Personal Access Token with `read:packages` permission as the password.

## Variable Override Hierarchy

Variables are loaded in this order (later takes precedence):

1. `group_vars/all.yml` - Global defaults
2. `group_vars/production.yml` - Production environment variables
3. `host_vars/localhost.yml` - Host-specific variables
4. `inventory.ini` vars sections - Inventory-defined variables
5. Command-line variables (`-e var=value`)

## Example Overrides

### Production Environment

```yaml
# group_vars/production.yml
irc_config:
  server: "irc.libera.chat"
  port: 6697
  use_tls: true
  channels: ["#production", "#private"]
  channel_keys:
    "#private": "{{ vault_private_channel_key | default('') }}"

container_options:
  restart_policy: unless-stopped
  network: bridge
```

### Development Environment

```yaml
# group_vars/development.yml
irc_config:
  server: "127.0.0.1"
  port: 6667
  channels: ["#dev", "#testing", "#private-dev"]
  channel_keys:
    "#private-dev": "dev-secret-key"
    "#testing": "test123"

build_options:
  force_rebuild: true
```

### Remote Deployment

```yaml
# host_vars/production.example.com.yml
app_user: sexo-prod
app_dir: "/opt/sexo"

irc_config:
  nickname: "sexo-prod"
  channels: ["#operations"]
```
