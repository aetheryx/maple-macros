use std::{thread::sleep, time::Duration};
use anyhow::*;

mod cv;
mod peripherals;
mod screen;

const SCREEN_PATH: &str = "/Users/zain/Development/maple-macros/data/screen.png";

fn main() -> Result<()> {
  screen::screencapture(SCREEN_PATH)?;

  let items = cv::get_item_coords()?;
  for item in items {
    println!("{item:?}");
    peripherals::mouse_click((item.x + 352) as f64, item.y as f64).unwrap();
    peripherals::mouse_click(1720.0, 100.0).unwrap();
    peripherals::send_key(0x0024).unwrap();
    sleep(Duration::from_millis(800));
  }

  Ok(())
}