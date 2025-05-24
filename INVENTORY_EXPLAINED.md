# Inventory Structure

The sexo IRC bot deployment uses a single, simple inventory file: `inventory.ini`

## Structure

```ini
[development]
localhost ansible_connection=local

[production]
# Add your production servers here when ready:
# prod-server1.example.com ansible_user=deploy ansible_ssh_private_key_file=~/.ssh/id_rsa
# prod-server2.example.com ansible_user=deploy

[development:vars]
ansible_python_interpreter=/usr/bin/python3

[production:vars]
ansible_python_interpreter=/usr/bin/python3
```

## Usage

Use the `--limit` flag or deployment script flags to target specific environments:

### Direct Ansible Commands
```bash
# Deploy to development
ansible-playbook -i inventory.ini --limit development deploy.yml

# Deploy to production
ansible-playbook -i inventory.ini --limit production deploy.yml
```

### Using deploy.sh Script
```bash
# Deploy to development (default)
./deploy.sh
./deploy.sh --dev

# Deploy to production
./deploy.sh --prod
```

## Adding Production Servers

When ready to deploy to production:

1. Edit `inventory.ini`
2. Add servers under the `[production]` group
3. Configure SSH keys and user access
4. Update `group_vars/production.yml` if needed

Example production server configuration:
```ini
[production]
server1.mycompany.com ansible_user=deploy ansible_ssh_private_key_file=~/.ssh/deploy_key
server2.mycompany.com ansible_user=deploy ansible_ssh_private_key_file=~/.ssh/deploy_key
```

## Variable Hierarchy

Variables are loaded in this order (later overrides earlier):
1. `group_vars/all.yml` - Global defaults
2. `group_vars/development.yml` or `group_vars/production.yml` - Environment-specific
3. `host_vars/hostname.yml` - Host-specific overrides
4. Inventory variables
5. Command-line variables (`-e` flag)

This gives you maximum flexibility while keeping the inventory file simple and clean.
4. **Follows conventions** - inventory.ini is the default

This is much more intuitive! 🎉
