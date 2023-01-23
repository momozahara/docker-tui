use std::process::{Command, Output};

pub fn up(username: String, hostname: String, path: String, target: Option<String>) -> Output {
    let output = Command::new("ssh")
        .arg("-tt")
        .arg(format!("{}@{}", username, hostname))
        .arg(format!("cd {}", path))
        .arg("&& docker compose up")
        .arg("-d")
        .arg(match target {
            Some(_target) => {
                format!("--build {}", _target)
            }
            None => "".to_owned(),
        })
        .output()
        .expect("command failed to start");

    output
}

pub fn down(username: String, hostname: String, path: String, rmi: Option<String>) -> Output {
    let output = Command::new("ssh")
        .arg("-tt")
        .arg(format!("{}@{}", username, hostname))
        .arg(format!("cd {}", path))
        .arg("&& docker compose down")
        .arg(match rmi {
            Some(r) => format!("--rmi {}", r),
            None => String::from(""),
        })
        .output()
        .expect("command failed to start");

    output
}

pub fn start(username: String, hostname: String, path: String, target: Option<String>) -> Output {
    let output = Command::new("ssh")
        .arg("-tt")
        .arg(format!("{}@{}", username, hostname))
        .arg(format!("cd {} && docker compose start", path))
        .arg(match target {
            Some(_target) => {
                format!("{}", _target)
            }
            None => "".to_owned(),
        })
        .output()
        .expect("command failed to start");

    output
}

pub fn stop(username: String, hostname: String, path: String, target: Option<String>) -> Output {
    let output = Command::new("ssh")
        .arg("-tt")
        .arg(format!("{}@{}", username, hostname))
        .arg(format!("cd {} && docker compose stop", path))
        .arg(match target {
            Some(_target) => {
                format!("{}", _target)
            }
            None => "".to_owned(),
        })
        .output()
        .expect("command failed to start");

    output
}
