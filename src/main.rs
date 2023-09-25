use std::env;

fn main() {
    let mut args = env::args().skip(1);
    let command: &str = &args.next().unwrap();
    match command {
        "git-release" => perform_git_release(args),
        _ => ()
    };
}

fn perform_git_release(mut args: impl Iterator<Item = &str>) {
    //TODO implement
}
