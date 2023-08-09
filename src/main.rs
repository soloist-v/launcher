#![windows_subsystem = "windows"]

use std::process::{Command, Stdio};
use serde::Deserialize;

#[derive(Deserialize)]
struct Cfg {
    executable: String,
    args: Vec<String>,
    lib_dir: String,
    output: String,
    keep_alive: bool,
}

fn load_cfg(path: &str) -> Result<Cfg, anyhow::Error> {
    let content = std::fs::read(path)?;
    let content = String::from_utf8(content)?;
    let cfg: Cfg = serde_yaml::from_str(content.as_str())?;
    Ok(cfg)
}

fn make_stdout(path: &str) -> Result<(Stdio, Stdio), anyhow::Error> {
    if path.is_empty() {
        Ok((Stdio::inherit(), Stdio::inherit()))
    } else {
        let output_file = std::fs::File::create(path)?;
        let out = output_file;
        Ok((Stdio::from(out.try_clone()?), Stdio::from(out)))
    }
}

fn main() -> Result<(), anyhow::Error> {
    let cfg = load_cfg("cfg.yaml")?;
    let (out, err) = make_stdout(cfg.output.as_str())?;
    let current_path = std::env::var("PATH").unwrap_or_else(|_| String::new());
    let new_path = format!("{};{}", cfg.lib_dir, current_path);
    let mut proc = Command::new(cfg.executable)
        .stdout(out)
        .stderr(err)
        .env("PATH", new_path)
        .args(cfg.args).spawn()?;
    if cfg.keep_alive {
        let res = proc.wait()?;
        std::process::exit(res.code().unwrap_or(0));
    }
    Ok(())
}
