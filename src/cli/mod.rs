use std::{
    io::{self, Write},
    process::{Command, Stdio},
};

pub fn up(username: String, hostname: String, path: String, target: Option<String>) {
    let output = Command::new("ssh")
        .stdin(Stdio::piped())
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
        .spawn()
        .expect("command failed to start");

    let output = output.wait_with_output().unwrap();

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}

pub fn down(username: String, hostname: String, path: String, rmi: Option<String>) {
    let output = Command::new("ssh")
        .arg("-tt")
        .arg(format!("{}@{}", username, hostname))
        .arg(format!("cd {}", path))
        .arg("&& docker compose down")
        .arg(match rmi {
            Some(r) => format!("--rmi {}", r),
            None => String::from(""),
        })
        .spawn()
        .expect("command failed to start");

    let output = output.wait_with_output().unwrap();

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}

pub fn start(username: String, hostname: String, path: String, target: Option<String>) {
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
        .spawn()
        .expect("command failed to start");

    let output = output.wait_with_output().unwrap();

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}

pub fn stop(username: String, hostname: String, path: String, target: Option<String>) {
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
        .spawn()
        .expect("command failed to start");

    let output = output.wait_with_output().unwrap();

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}
