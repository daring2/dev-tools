use std::env;
use std::os::windows::process::CommandExt;
use std::process::Command;

use anyhow::{anyhow, bail, Context, Result};
use clap::{Args, Parser, Subcommand};

use crate::gradle_props::GradleProperties;

mod gradle_props;

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
    /// Clean project before release
    #[arg(short, long)]
    clean: bool,
    /// Build project before release
    #[arg(short, long)]
    build: bool,
    //TODO add options current_version, clean, build, etc
}

type CmdResult<T> = Result<T>;
// type CmdResult<T> = Result<T, String>;

fn main() -> CmdResult<()> {
    execute()
}

fn execute() -> CmdResult<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Release(args) => perform_release(args)
    }
}

fn perform_release(args: ReleaseArgs) -> CmdResult<()> {
    let mut gradle_props = GradleProperties::load("gradle.properties")?;
    let current_version = gradle_props.get("version")
        .map(|it|it.to_string())
        .context("Cannot find current version property")?;
    let next_version = args.next_version.trim();
    if next_version.is_empty() {
        bail!("Please specify next version");
        // return Err("Please specify next version".to_string());
    }
    println!("release: current_version={current_version}, next_version={next_version}");
    let gradle_cmd = build_gradle_command(&args);
    exec_cmd(&format!("{gradle_cmd} publish"))?;
    exec_cmd(&format!("git tag -a v{0} -m v{0}", current_version))?;
    gradle_props.set("version", &next_version);
    gradle_props.save()?;
    exec_cmd(&format!("git commit -am \"build version {current_version}\""))?;
    Ok(())
}

fn build_gradle_command(args: &ReleaseArgs) -> String {
    let mut gradle_args = vec!["gradlew.bat"];
    gradle_args.push("--no-daemon");
    if args.clean {
        gradle_args.push("clean");
    }
    if args.build {
        gradle_args.push("build");
    }
    return gradle_args.join(" ");
}

fn exec_cmd(command: &str) -> CmdResult<()> {
    let current_dir = env::current_dir()
        .context("Cannot get current_dir")?;
        // .map_err(|e|format!("Cannot get current_dir: {e}"))?;
    println!("execute {command}");
    Command::new("cmd").arg("/C")
        .raw_arg(command)
        .current_dir(current_dir)
        .status()
        .map_err(|e|anyhow!(e))
        // .map_err(|e|e.to_string())
        .and_then(|it| match it {
            _ if it.success() => Ok(it),
            _ => Err(anyhow!(it))
        })
        .with_context(||format!("Cannot execute command '{command}'"))?;
        // .map_err(|e|format!("Cannot execute command '{command}': {e}"))?;
    Ok(())
}
