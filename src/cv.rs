use opencv::prelude::*;
use anyhow::*;
use opencv::{imgproc, imgcodecs, core, highgui};

const INVENTORY_W: i32 = 16;
const INVENTORY_H: i32 = 8;
const INVENTORY_ITEM_S: i32 = 126;

fn read_img(name: &str) -> Result<Mat> {
  let mat = imgcodecs::imread(
    &format!("/Users/zain/Development/maple-macros/data/{}", name),
    imgcodecs::IMREAD_COLOR
  )?;

  Ok(mat)
}

fn find_inventory(view: &mut Mat) -> Result<core::Point> {
  let template = read_img("inventory-header.png")?;

  let matches = {
    let mut result = Mat::default();
    let mask = Mat::default();
    imgproc::match_template(view, &template, &mut result, imgproc::TM_CCORR_NORMED, &mask)?;
    result
  };

  let mut max_loc = core::Point::default();
  let mask = Mat::default();
  core::min_max_loc(&matches, None, None, None, Some(&mut max_loc), &mask)?;

  Ok(core::Point::new(
    max_loc.x + 38,
    max_loc.y + template.rows() + 15
  ))
}

fn inventory_item_matches(x: i32, y: i32, inventory: &core::Point, matches: &Mat) -> Result<bool> {
  let rect = core::Rect::new(
    inventory.x + (x * INVENTORY_ITEM_S),
    inventory.y + (y * INVENTORY_ITEM_S),
    100,
    100
  );

  let roi = Mat::roi(matches, rect)?;
  let mut max_val = 0.0;
  let mask = Mat::default();
  core::min_max_loc(&roi, None, Some(&mut max_val), None, None, &mask)?;

  let found = max_val >= 0.98;
  Ok(found)
}

pub fn get_item_coords() -> Result<Vec<core::Point>> {
  let mut view = read_img("screen.png")?;
  let matches = {
    let template = read_img("red-scroll.png")?;
    let mut result = Mat::default();
    let mask = Mat::default();
    imgproc::match_template(&view, &template, &mut result, imgproc::TM_CCORR_NORMED, &mask)?;
    result
  };

  let inventory = find_inventory(&mut view)?;
  let points = (0..INVENTORY_W)
    .map(|x| (0..INVENTORY_H).map(move |y| (x, y)))
    .flatten()
    .filter(|(x, y)| inventory_item_matches(*x, *y, &inventory, &matches).unwrap_or(false))
    .map(|(x, y)| core::Point::new(
      (inventory.x + (x * INVENTORY_ITEM_S) + (INVENTORY_ITEM_S / 2)) / 2,
      (inventory.y + (y * INVENTORY_ITEM_S) + (INVENTORY_ITEM_S / 2)) / 2
    ));

  println!("{:?}", view.size()?);

  // for point in points.clone() {
  //   let rect = core::Rect::new(point.x, point.y, 126, 126);
  //   imgproc::rectangle(&mut view, rect, (0.0, 0.0, 255.0).into(), -1, imgproc::LINE_8, 0)?;
  // }

  // highgui::imshow("view", &view)?;
  // highgui::wait_key_def()?;

  Ok(points.collect())
}
