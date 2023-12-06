use std::collections::HashMap;
use std::env;
use std::fs;
use std::os::windows::process::CommandExt;
use std::process::Command;

use anyhow::{anyhow, bail, Context, Result};
use clap::{Args, Parser, Subcommand};

mod gradle_props;
use crate::gradle_props::GradleProperties;

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
    let mut gradle_props = load_gradle_properties()?;
    let current_version = gradle_props.get("version")
        .map(|it|it.to_string())
        .context("Cannot find current version property")?;
    let next_version = args.next_version;
    if next_version.trim().is_empty(){
        bail!("Please specify next version");
        // return Err("Please specify next version".to_string());
    }
    println!("release: current_version={current_version}, next_version={next_version}");
    //TODO add option for clean build
    let gradle_cmd = "gradlew.bat --no-daemon";
    exec_cmd(&format!("{gradle_cmd} publish"))?;
    exec_cmd(&format!("git tag -a v{0} -m v{0}", current_version))?;
    gradle_props.values.insert("version".to_string(), next_version);
    //TODO keep properties order
    write_gradle_properties(gradle_props)?;
    //TODO use "build version {current_version} message
    exec_cmd(&format!("git commit -am \"build version {current_version}\""))?;
    Ok(())
}



fn load_gradle_properties() -> CmdResult<GradleProperties> {
    let mut props = GradleProperties {
        keys: Vec::new(),
        values: HashMap::new(),
    };
    let file = "gradle.properties";
    let content = fs::read_to_string(file)
        .with_context(||format!("Cannot read file '{file}'"))?;
    for line in content.lines() {
        let (key, value) = line.split_once("=")
            .unwrap_or(("", ""));
        let key = key.trim().to_string();
        let value = value.trim().to_string();
        if !key.is_empty() {
            props.keys.push(key.to_string()); //TODO optimize
            props.values.insert(key, value);
        }
    }
    return Ok(props)
}

fn write_gradle_properties(props: GradleProperties) -> CmdResult<()> {
    let content = props.keys.iter()
        .map(|key| {
            let value = props.get(key).unwrap_or("");
            format!("{}={}", key, value)
        })
        .collect::<Vec<String>>()
        .join("\n");
    fs::write("gradle.properties", content)
        .context("Cannot update version")?;
    Ok(())
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
