const TEAM_URL: &str = "https://team-api.infra.rust-lang.org/v1/teams/all.json";

use serde::Deserialize;
use eyre::*;
use std::process::Command;
use std::process::Output;

#[derive(Deserialize)]
struct All {
    members: Vec<Person>,
}

#[derive(Deserialize)]
struct Person {
    github: String,
}

fn cmd(cmd: &str, args: &[&str]) -> std::io::Result<Output> {
    Command::new(cmd)
        .args(args)
        .output()
}

fn main() -> Result<()> {
    let username = std::env::args()
        .skip(1)
        .next()
        .ok_or_else(|| eyre!("should have passed a username argument"))?;

    let all = reqwest::blocking::get(TEAM_URL)?.json::<All>()?;
    for person in all.members {
        if person.github == username {
            // Check if user exists
            let id = cmd("id", &[&username])?;
            if id.status.success() {
                let id = String::from_utf8(id.stdout)?;
                let id: u64 = id.parse()?;
                ensure!(id > 1000, "cannot login with system user");
            } else {
                // If user does not exist, create it
                ensure!(
                    cmd("useradd", &["--create-home", &username])?.status.success(),
                    "failed to create user"
                );
            }
            // Get the keys the user added to their github account
            let keys =
                reqwest::blocking::get(format!("https://github.com/{}.keys", username))?.text()?;
            println!("{}", keys);
            return Ok(());
        }
    }
    bail!("user not found");
}
