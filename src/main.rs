use std::{env, io};
use std::fs;
use std::process::Command;

fn main() {
    let mut args = env::args().skip(1);
    let command: &str = &args.next().unwrap();
    match command {
        "release" => perform_release(args),
        _ => ()
    };
}

fn perform_release(mut args: impl Iterator<Item = String>) {
    let current_version = read_current_version();
    let next_version = args.next().unwrap();
    println!("release: current_version={}, next_version={}", current_version, next_version);
    exec_cmd("gradlew.bat clean build publish");
}

fn read_current_version() -> String {
    let content = fs::read_to_string("gradle.properties").unwrap();
    let version = content.strip_prefix("version=").unwrap().trim();
    return String::from(version)
}

fn exec_cmd(command: &str) {
    let status = Command::new("cmd")
        .args(["/C", command])
        .current_dir(env::current_dir().unwrap())
        .status()
        .unwrap();
    assert!(status.success());
}
