#!/usr/bin/env bash

set -ex

# In Rust We Trust
sudo apt install cargo pkg-config libssl-dev

# Build login program
cd team_login
cargo build --all
cd ..

# Set up the auto-user creation
sudo cp team_login/target/debug/cronjob /etc/cron.create_all_team_users
sudo cp crontab_append /etc/cron.d/create_all_team_users

# Set up the auto-login via ssh
sudo cp team_login/target/debug/team_login /etc/ssh/team_login
sudo cat sshd_append >> /etc/sshd_config

# The files that initially appear in a user's home dir
sudo cp skel/* /etc/skel/

# Disable existing message of the day
sudo chmod -x /etc/update-motd.d/*

# Add our own message of the day

sudo cp welcome-rust-dev /etc/update-motd.d/01-welcome-rust-dev