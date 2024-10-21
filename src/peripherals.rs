use std::{thread::sleep, time::Duration};

use core_graphics::{
  event::{CGEvent, CGEventTapLocation, CGEventType, CGMouseButton},
  event_source::{CGEventSource, CGEventSourceStateID},
  geometry::CGPoint,
};

pub fn mouse_click(x: f64, y: f64) -> Result<(), ()> {
  let point = CGPoint::new(x, y);
  for event in [CGEventType::MouseMoved, CGEventType::LeftMouseDown, CGEventType::LeftMouseUp] {
    let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState)?;
    let event = CGEvent::new_mouse_event(source, event, point, CGMouseButton::Left)?;
    event.post(CGEventTapLocation::HID);
    sleep(Duration::from_millis(50));
  }

  sleep(Duration::from_millis(100));

  Ok(())
}

pub fn send_key(key_code: u16) -> Result<(), ()> {
  for keydown in [true, false] {
    let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState)?;
    let event = CGEvent::new_keyboard_event(source, key_code, keydown)?;
    event.post(CGEventTapLocation::HID);
    sleep(Duration::from_millis(50));
  }

  Ok(())
}