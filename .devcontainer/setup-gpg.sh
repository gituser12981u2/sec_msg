#!/bin/bash
set -e

echo "Starting GPG setup"

# Ensure correct permissions for .gnupg directory
mkdir -p ~/.gnupg
chmod 700 ~/.gnupg
find ~/.gnupg -type f -exec chmod 600 {} \;
find ~/.gnupg -type d -exec chmod 700 {} \;
echo "Permissions set"

# Remove GPG agent socket files to properly reset gpg-agent with new permissions
rm -rf /home/vscode/.gnupg/S.gpg-agent*

# Function to get Git config
get_git_config() {
    local key=$1
    git config --global --get "$key"
}

# Generate a new GPG key if one doesn't exist
if ! gpg --list-secret-keys --keyid-format LONG | grep sec > /dev/null; then
    echo "Generating a new GPG key..."
    
    # Get Git user name and email
    GIT_USER_NAME=$(get_git_config user.name)
    GIT_USER_EMAIL=$(get_git_config user.email)

    if [ -z "$GIT_USER_NAME" ] || [ -z "$GIT_USER_EMAIL" ]; then
        echo "Error: Git user name or email is not set. Please run the Git configuration script manually."
        exit 1
    fi

    # Generate the key
    gpg --batch --generate-key <<EOF
%echo Generating a project-specific OpenPGP key
Key-Type: EDDSA
Key-Curve: ed25519
Key-Usage: sign
Subkey-Type: ECDH
Subkey-Curve: cv25519
Subkey-Usage: encrypt
Name-Real: $GIT_USER_NAME
Name-Email: $GIT_USER_EMAIL
Expire-Date: 0
%no-protection
%commit
%echo done
EOF
fi
echo "GPG key generated"

# Configure Git to use GPG
git config --global commit.gpgsign true
KEY_ID=$(gpg --list-secret-keys --keyid-format LONG | grep sec | awk '{print $2}' | cut -d'/' -f2)
git config --global user.signingkey ${KEY_ID}

echo "GPG setup complete. Your signing key is: ${KEY_ID}"
echo "Please add the following public key to your GitHub account:"
gpg --armor --export ${KEY_ID}