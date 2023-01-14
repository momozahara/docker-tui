use dirs::home_dir;
use dotenv;
use std::{env, fs, io::Write};

#[test]
fn test_load_name() {
    let files = load_name();

    assert_eq!(files.len(), 1);

    for file in files {
        println!("{}", file);
    }
}

pub fn load_name() -> Vec<String> {
    let target_path = home_dir()
        .and_then(|a| Some(a.join("pcode-cli/docker")))
        .unwrap();
    let path = target_path.as_path();

    let mut files: Vec<String> = Vec::new();

    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let name = entry.file_name();
        let file_path = entry.path();

        if file_path.extension() != Some("env".as_ref()) {
            continue;
        }

        let mut name = name.into_string().unwrap();
        let name_split: Vec<&str> = name.split('.').collect();
        name = String::from(name_split[0]);

        files.push(name);
    }

    files
}

pub fn load(profile: String) -> Vec<String> {
    let target_path = home_dir()
        .and_then(|a| Some(a.join(format!("pcode-cli/docker/{}.env", profile))))
        .unwrap();

    env::remove_var("USERNAME");
    env::remove_var("HOSTNAME");
    env::remove_var("TARGET_PATH");

    dotenv::from_path(target_path.as_path()).expect("profile does not existed");

    let username = env::var("USERNAME").unwrap();
    let hostname = env::var("HOSTNAME").unwrap();
    let path = env::var("TARGET_PATH").unwrap();

    vec![username, hostname, path]
}

pub fn create(profile: String, username: String, hostname: String, c_path: String) {
    let target_path = home_dir()
        .and_then(|a| Some(a.join(format!("pcode-cli/docker/{}.env", profile))))
        .unwrap();
    let path = target_path.as_path();
    let prefix = path.parent().unwrap();

    fs::create_dir_all(prefix).unwrap();
    fs::remove_file(path).unwrap_or(());

    let mut file = fs::File::create(path).unwrap();
    file.write(
        format!(
            "USERNAME={}\nHOSTNAME={}\nTARGET_PATH={}",
            username, hostname, c_path
        )
        .as_bytes(),
    )
    .unwrap();
}

pub fn remove(profile: String) {
    let target_path = home_dir()
        .and_then(|a| Some(a.join(format!("pcode-cli/docker/{}.env", profile))))
        .unwrap();
    let path = target_path.as_path();
    let prefix = path.parent().unwrap();

    fs::create_dir_all(prefix).unwrap();
    fs::remove_file(path).unwrap_or(());
}
