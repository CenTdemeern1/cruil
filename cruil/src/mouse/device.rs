//! Mouse support is still currently work-in-progress and a big hack

use super::*;
use crate::*;
use hidapi::{HidDevice, MAX_REPORT_DESCRIPTOR_SIZE};

pub struct MouseDevice {
    device: HidDevice,
    last_pressed: MouseButtons,
    buffer: [u8; MAX_HID_PACKET_SIZE],
}

impl MouseDevice {
    pub(crate) fn new(device: HidDevice) -> Self {
        // let mut buf = [0; MAX_REPORT_DESCRIPTOR_SIZE];
        // let read = device.get_report_descriptor(&mut buf).unwrap();
        // println!("{:X?}", &buf[..read]);
        // panic!("Stop");
        Self {
            buffer: [0; _],
            last_pressed: Default::default(),
            device,
        }
    }

    /// Returns whether the given mouse buttons are pressed.
    pub fn are_buttons_pressed(&self, buttons: MouseButtons) -> bool {
        self.last_pressed.contains(buttons)
    }
}

impl ReadableDevice for MouseDevice {
    type State = MouseInputState;

    fn read_raw(&mut self, blocking: bool) -> CruilResult<&[u8]> {
        self.device.set_blocking_mode(blocking)?;
        let read = self.device.read(&mut self.buffer)?;
        let report = if read != 0 {
            &self.buffer[1..read] // Skip report ID
        } else {
            &self.buffer[..0]
        };
        Ok(report)
    }

    fn read(&mut self, blocking: bool) -> CruilResult<Self::State> {
        let report = self.read_raw(blocking)?;
        let report_length = report.len();

        if report_length == 0 {
            // Gracefully handle no response by returning last known state
            return Ok(MouseInputState {
                currently_pressed: self.last_pressed.clone(),
                ..Default::default()
            });
        }

        if report_length < 4 {
            return Err(CruilError::ProtocolViolation(
                ProtocolViolation::ResponseTooShort(report_length),
            ));
        }

        let currently_pressed = MouseButtons::from_bits_retain(report[0]);
        let delta_x = (i16::from_le_bytes([report[1], report[2]]) << 4) >> 4;
        let delta_y = i16::from_le_bytes([report[2], report[3]]) >> 4;
        let delta_wheel = report.get(4).copied().unwrap_or_default().cast_signed();

        let just_pressed = currently_pressed.difference(self.last_pressed);
        let just_released = self.last_pressed.difference(currently_pressed);

        self.last_pressed = currently_pressed.clone();

        Ok(MouseInputState {
            delta_x,
            delta_y,
            currently_pressed,
            just_pressed,
            just_released,
            delta_wheel,
        })
    }
}
