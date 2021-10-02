const TEAM_URL: &str = "https://team-api.infra.rust-lang.org/v1/teams/all.json";

use serde::Deserialize;

#[derive(Deserialize)]
struct All {
    members: Vec<Person>,
}

#[derive(Deserialize)]
struct Person {
    github: String,
}

fn cmd(cmd: &str, args: &[&str]) -> Result<bool, Box<dyn std::error::Error>> {
    use std::process::Command;
    use std::process::Stdio;
    Ok(Command::new(cmd)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .args(args)
        .status()?
        .success())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let username = std::env::args()
        .skip(1)
        .next()
        .expect("should have passed a username argument");

    // Yes there is a github user called "root",
    // no it's not an official one by github.
    assert_ne!(username, "root", "sorry bud, not happening");

    let all = reqwest::blocking::get(TEAM_URL)?.json::<All>()?;
    for person in all.members {
        if person.github == username {
            // Check if user exists
            if !cmd("id", &[&username])? {
                // If user does not exist, create it
                assert!(
                    cmd("useradd", &["--create-home", &username])?,
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
    panic!("user not found");
}
