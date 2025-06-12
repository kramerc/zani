# GitHub Actions Deployment Setup

This document describes the GitHub Actions workflows for automated deployment to production.

## Workflows

### 1. Build and Push Docker Image (`deploy-image.yml`)
- Triggers on pushes to `main` branch
- Builds the Docker image and pushes to GitHub Container Registry (GHCR)
- Uses `GITHUB_TOKEN` for authentication to GHCR

### 2. Deploy to Production (`deploy-production.yml`)
- Triggers automatically when the image build workflow completes successfully
- Can also be triggered manually via `workflow_dispatch`
- Uses Ansible to deploy the latest image to production servers

## Prerequisites

### Repository Secrets
The following secrets must be configured in your GitHub repository:

1. **`DEPLOY_KEY`** - Private SSH key for accessing production servers
   - Generate with: `ssh-keygen -t ed25519 -f deploy_key -C "github-actions"`
   - Add the public key (`deploy_key.pub`) to the `~/.ssh/authorized_keys` on production servers
   - Add the private key (`deploy_key`) content as the `DEPLOY_KEY` repository secret

### Environment Variables
The deployment workflow automatically sets these environment variables:
- `GHCR_USERNAME` - Set to `${{ github.actor }}` (the GitHub user who triggered the action)
- `GHCR_TOKEN` - Set to `${{ secrets.GITHUB_TOKEN }}` (automatically provided by GitHub)

## Production Configuration

The production deployment uses the following configuration:
- **Target server**: `nexus.mer.st`
- **Deploy user**: `deploy`
- **Container registry**: `ghcr.io`
- **Image**: `ghcr.io/kramerc/zani:main`

## Manual Deployment

To manually trigger a deployment:
1. Go to the "Actions" tab in your GitHub repository
2. Select the "Deploy to Production" workflow
3. Click "Run workflow"
4. Choose the branch and click "Run workflow"

## Troubleshooting

### SSH Connection Issues
- Ensure the `DEPLOY_KEY` secret contains the correct private key
- Verify the corresponding public key is in `~/.ssh/authorized_keys` on the target server
- Check that the `deploy` user exists on the target server

### Registry Authentication Issues
- The workflow uses `GITHUB_TOKEN` which should have sufficient permissions for package access
- Ensure the GitHub repository has access to the container registry

### Ansible Issues
- Check the `inventory.ini` file for correct server hostnames and SSH configuration
- Verify the `requirements.yml` file contains all necessary Ansible collections
