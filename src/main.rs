use std::env;
use std::fs;
use std::process::Command;

fn main() {
    let mut args = env::args().skip(1);
    //TODO replace panic with Result
    let command: &str = &args.next()
        .expect("Please specify command");
    match command {
        "release" => perform_release(args),
        _ => panic!("Invalid command '{command}'")
    };
}

fn perform_release(mut args: impl Iterator<Item = String>) {
    let current_version = load_current_version()
        .expect("Cannot load current version");
    let next_version = args.next()
        .expect("Please specify next version");
    println!("release: current_version={}, next_version={}", current_version, next_version);
    //TODO add option for clean build
    let gradle_cmd = "gradlew.bat --no-daemon";
    exec_cmd(&format!("{gradle_cmd} publish"));
    exec_cmd(&format!("git tag -a v{0} -m \"v{0}\"", current_version));
    fs::write("gradle.properties", format!("version={next_version}"))
        .expect("Cannot update version");
    // exec_cmd("git commit push");
}

fn load_current_version() -> Option<String> {
    let file = "gradle.properties";
    fs::read_to_string(file)
        .expect("Cannot read file '{file}'")
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
