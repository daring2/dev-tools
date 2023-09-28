use std::env;
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
    let current_version = read_current_version().unwrap();
    let next_version = args.next().unwrap();
    println!("release: current_version={}, next_version={}", current_version, next_version);
    exec_cmd("gradlew.bat clean build publish");
    exec_cmd(&format!("git tag -a v{0} -m \"v{0}\"", current_version));
    // exec_cmd("git push");
    fs::write("gradle.properties", format!("version={next_version}")).unwrap();
}

fn read_current_version() -> Option<String> {
    fs::read_to_string("gradle.properties").unwrap()
        .lines()
        .filter_map(|it|it.strip_prefix("version="))
        .map(|it|String::from(it.trim()))
        .next()
}

fn exec_cmd(command: &str) {
    println!("execute {command}");
    let status = Command::new("cmd")
        .args(["/C", command])
        .current_dir(env::current_dir().unwrap())
        .status()
        .unwrap();
    assert!(status.success());
}
