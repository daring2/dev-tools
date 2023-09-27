use std::env;
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
    let new_version = args.next().unwrap();
    Command::new("cmd")
        .args(["/C", "test.bat p1 p2"])
        .current_dir(env::current_dir().unwrap())
        .status()
        .unwrap();
}
