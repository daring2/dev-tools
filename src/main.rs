use std::env;

fn main() {
    let mut args = env::args().skip(1);
    let command: &str = &args.next().unwrap();
    match command {
        "release" => perform__release(args),
        _ => ()
    };
}

fn perform__release(mut args: impl Iterator<Item = String>) {
    let new_version = args.next().unwrap();
    println!("new version: {new_version}");
}
