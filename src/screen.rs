use std::process::Command;
use anyhow::*;

pub fn screencapture(path: &str) -> Result<()> {
  let status = Command::new("screencapture")
    .arg("-m")
    .arg("-R").arg("352,0,2048,1152") // x, y, w, h
    .arg(path)
    .status()?;

  ensure!(status.success());

  Ok(())
}