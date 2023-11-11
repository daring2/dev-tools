use std::{env, process};
use std::fs;
use std::os::windows::process::CommandExt;
use std::process::Command;
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Release(ReleaseArgs),
}

#[derive(Args)]
struct ReleaseArgs {
    /// next version
    next_version: String,
}

fn main() {
    let result = execute();
    if let Err(e) = result {
        eprintln!("{e}");
        process::exit(1);
    }
}

type CmdResult<T> = Result<T, String>;

fn execute() -> CmdResult<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Release(args) => perform_release(args)
    }
}

fn perform_release(args: ReleaseArgs) -> CmdResult<()> {
    let current_version = load_current_version()?;
    let next_version = args.next_version;
    if next_version.trim().is_empty(){
        return Err("Please specify next version".to_string());
    }
    println!("release: current_version={current_version}, next_version={next_version}");
    //TODO add option for clean build
    let gradle_cmd = "gradlew.bat --no-daemon";
    exec_cmd(&format!("{gradle_cmd} publish"))?;
    exec_cmd(&format!("git tag -a v{0} -m v{0}", current_version))?;
    fs::write("gradle.properties", format!("version={next_version}"))
        .map_err(|e|format!("Cannot update version: {e}"))?;
    //TODO use "build version {current_version} message
    exec_cmd(&format!("git commit -am \"build version {current_version}\""))?;
    Ok(())
}

fn load_current_version() -> CmdResult<String> {
    let file = "gradle.properties";
    fs::read_to_string(file)
        .map_err(|e|format!("Cannot read file '{file}': {e}"))?
        .lines()
        .filter_map(|it|it.strip_prefix("version="))
        .map(|it|String::from(it.trim()))
        .next()
        .ok_or(format!("Cannot find current version property"))
}

fn exec_cmd(command: &str) -> CmdResult<()> {
    let current_dir = env::current_dir().map_err(
        |e|format!("Cannot get current_dir: {e}")
    )?;
    println!("execute {command}");
    Command::new("cmd").arg("/C")
        .raw_arg(command)
        .current_dir(current_dir)
        .status()
        .map_err(|e|e.to_string())
        .and_then(|it| match it {
            _ if it.success() => Ok(it),
            _ => Err(it.to_string())
        })
        .map_err(|e|
            format!("Cannot execute command '{command}': {e}")
        )?;
    Ok(())
}
