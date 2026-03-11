//! Mouse support is still currently work-in-progress and a big hack

use super::*;
use crate::*;
use hidapi::HidDevice;

/// A mouse device. Represents a HID mouse.
///
/// This is usually equivalent to a single physical mouse, but any singular physical USB device could pretend to be any number of mice.
pub struct MouseDevice {
    device: HidDevice,
    last_pressed: MouseButtons,
    buffer: [u8; MAX_HID_PACKET_SIZE],
}

impl MouseDevice {
    pub(crate) fn new(device: HidDevice) -> Self {
        Self {
            buffer: [0; _],
            last_pressed: Default::default(),
            device,
        }
    }

    /// Returns whether the given mouse buttons are pressed in the internal state.
    ///
    /// The internal state updates after any calls to [`read`](Self::read) or [`Self::try_read`].
    pub fn are_buttons_pressed(&self, buttons: MouseButtons) -> bool {
        self.last_pressed.contains(buttons)
    }

    pub(crate) fn read_internal_buffer(&mut self, blocking: bool) -> CruilResult<usize> {
        self.device.set_blocking_mode(blocking)?;
        Ok(self.device.read(&mut self.buffer)?)
    }

    /// This is an implementation to parse one type of mouse which I hope is common.
    /// It does not correctly parse my daily driver mouse.
    /// To properly parse mice, I think I need to parse the report descriptor.
    /// This might be good for keyboards too(?), we'll have to see when I get there.
    fn parse_internal_buffer(&mut self, report_length: usize) -> CruilResult<MouseInputState> {
        if report_length == 0 {
            // Gracefully handle no response by returning last known state
            return Ok(MouseInputState {
                currently_pressed: self.last_pressed.clone(),
                ..Default::default()
            });
        }

        let report = &self.buffer[1..report_length]; // Skip report ID

        if report_length < 4 {
            return Err(CruilError::ProtocolViolation(
                ProtocolViolation::ResponseTooShort(report_length),
            ));
        }

        let currently_pressed = MouseButtons::from_bits_retain(report[0]); // 8 bits of buttons
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

impl ReadableDevice for MouseDevice {
    type State = MouseInputState;

    fn read_raw(&self, buffer: &mut [u8], blocking: bool) -> CruilResult<usize> {
        self.device.set_blocking_mode(blocking)?;
        Ok(self.device.read(buffer)?)
    }

    fn try_read(&mut self) -> CruilResult<Option<Self::State>> {
        let report_length = self.read_internal_buffer(false)?;

        if report_length == 0 {
            return Ok(None);
        }

        self.parse_internal_buffer(report_length).map(Some)
    }

    fn read(&mut self, blocking: bool) -> CruilResult<Self::State> {
        let report_length = self.read_internal_buffer(blocking)?;
        self.parse_internal_buffer(report_length)
    }
}

impl IntoIterator for MouseDevice {
    type IntoIter = OwnedReadableDeviceIter<Self>;
    type Item = CruilResult<MouseInputState>;

    #[doc(alias = "owned_iter")]
    fn into_iter(self) -> Self::IntoIter {
        self.owned_iter()
    }
}
