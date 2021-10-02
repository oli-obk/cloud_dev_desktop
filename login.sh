#!/usr/bin/env bash

if curl -s https://team-api.infra.rust-lang.org/v1/teams/all.json | jq -r --arg user "${1}" '.members[].github | select(. == $user)'; then
    if ! id "$1" &>/dev/null; then
        useradd --create-home "$1"
    fi
    curl -s "https://github.com/$1.keys"
fi