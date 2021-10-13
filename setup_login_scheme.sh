#!/usr/bin/env bash

set -ex

# In Rust We Trust
sudo apt install cargo pkg-config libssl-dev

# Build login program
cd team_login
cargo build
cd ..

# Set up the auto-login via ssh
cp team_login/target/debug/team_login /etc/ssh/team_login
cat sshd_append >> /etc/sshd_config

# The files that initially appear in a user's home dir
cp skel/* /etc/skel/

# Disable existing message of the day
chmod -x /etc/update-motd.d/*

# Add our own message of the day

cp welcome-rust-dev /etc/update-motd.d/01-welcome-rust-dev