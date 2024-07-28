#!/bin/bash

echo "Checking GIT configuration"

# Source .env file if it exists
if [ -f .env ]; then
    export $(cat .env | xargs)
fi

# Use environment variables if set, otherwise prompt
if [ -n "$GIT_USER_NAME" ] && [ -n "$GIT_USER_EMAIL" ]; then
    git config --global user.name "$GIT_USER_NAME"
    git config --global user.email "$GIT_USER_EMAIL"
    echo "Git configuration set from environment variables:"
    echo "Name: $GIT_USER_NAME"
    echo "Email: $GIT_USER_EMAIL"
else
    if [ -z "$(git config --global user.name)" ] || [ -z "$(git config --global user.email)" ]; then
        echo "Git user name or email is not set. Please set them now:"
        read -p "Enter your Git user name: " git_user_name
        read -p "Enter your Git email: " git_user_email
        git config --global user.name "$git_user_name"
        git config --global user.email "$git_user_email"
        echo "Git configuration updated."
    else
        echo "Git user name and email are already set:"
        echo "Name: $(git config --global user.name)"
        echo "Email: $(git config --global user.email)"
    fi
fi