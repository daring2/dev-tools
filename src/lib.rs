use std::env;
use std::os::windows::process::CommandExt;
use std::process::Command;

use anyhow::{anyhow, bail, Context, Result};
use clap::{Args, Parser, Subcommand};

use crate::gradle_props::GradleProperties;

mod gradle_props;

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Release(ReleaseArgs),
}

#[derive(Args)]
pub struct ReleaseArgs {
    /// Next version
    next_version: String,
    /// Clean project before release
    #[arg(short, long)]
    clean: bool,
    /// Build project before release
    #[arg(short, long)]
    build: bool,
    /// Publish project
    #[arg(short, long, default_value_t = true)]
    publish: bool,
    /// Add VCS tag
    #[arg(long, default_value_t = true)]
    tag: bool,
    /// Commit changes into VCS
    #[arg(long, default_value_t = true)]
    commit: bool,
    //TODO add current_version option
}

type CmdResult<T> = Result<T>;
// type CmdResult<T> = Result<T, String>;

pub fn perform_release(args: ReleaseArgs) -> CmdResult<()> {
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
    exec_cmd(&gradle_cmd)?;
    if args.tag {
        exec_cmd(&format!("git tag -a v{0} -m v{0}", current_version))?;
    }
    gradle_props.set("version", &next_version);
    gradle_props.save()?;
    if args.commit {
        exec_cmd(&format!("git commit -am \"build version {current_version}\""))?;
    }
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
    if args.publish {
        gradle_args.push("publish");
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

#[cfg(test)]
mod tests;
