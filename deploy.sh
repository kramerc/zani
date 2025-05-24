#!/bin/bash

# Deploy sexo IRC bot using Ansible
set -e

echo "=== Deploying sexo IRC bot ==="

# Check if ansible is installed
if ! command -v ansible-playbook &> /dev/null; then
    echo "Error: ansible-playbook is not installed"
    echo "Please install Ansible first:"
    echo "  sudo apt install ansible  # Ubuntu/Debian"
    echo "  sudo dnf install ansible  # Fedora"
    echo "  pip install ansible       # pip"
    exit 1
fi

# Install required collections
echo "Installing required Ansible collections..."
ansible-galaxy collection install -r requirements.yml

# Determine target group
LIMIT="--limit development"  # Default to development

if [[ "$1" == "--dev" || "$1" == "--development" ]]; then
    LIMIT="--limit development"
    shift
elif [[ "$1" == "--prod" || "$1" == "--production" ]]; then
    LIMIT="--limit production"
    shift
fi

# Run the playbook
echo "Running deployment playbook..."
echo "Target: $LIMIT"
ansible-playbook -i inventory.ini $LIMIT deploy.yml "$@"

echo "=== Deployment complete ==="
echo ""
echo "Useful commands:"
echo "  Check status: systemctl --user status container-sexo"
echo "  View logs:    journalctl --user -u container-sexo -f"
echo "  Restart:      systemctl --user restart container-sexo"
echo "  Stop:         systemctl --user stop container-sexo"
echo ""
echo "Environment options:"
echo "  Development:  ./deploy.sh --dev (uses development group in inventory.ini)"
echo "  Production:   ./deploy.sh --prod (uses production group in inventory.ini)"
echo "  Default:      ./deploy.sh (deploys to development group)"
echo ""
echo "Production registry authentication:"
echo "  Set environment variables: GHCR_USERNAME and GHCR_TOKEN"
echo "  Example: export GHCR_USERNAME=myusername && export GHCR_TOKEN=ghp_xxxx"
echo ""
echo "Configuration:"
echo "  Variables:    See group_vars/ and host_vars/ directories"
echo "  Reference:    See VARIABLES.md for all available options"
echo "  Bot config:   Generated at /home/sexo/sexo/config/sexo.toml"
