use crate::{
    CruilError, CruilResult, KeySet, KeyboardInputState, ProtocolViolation, ReadableDevice,
    keys::{
        Modifiers,
        raw::{KEY_ERR_OVF, KEY_NONE},
    },
};
use hidapi::HidDevice;

/// Allegedly, full-speed HID packets are still at most 64 bytes.
const MAX_HID_PACKET_SIZE: usize = 64;

pub struct KeyboardDevice {
    device: HidDevice,
    last_pressed: KeySet,
    buffer: [u8; MAX_HID_PACKET_SIZE],
}

impl KeyboardDevice {
    pub(crate) fn new(device: HidDevice) -> Self {
        Self {
            buffer: [0; _],
            last_pressed: Default::default(),
            device,
        }
    }
}

impl ReadableDevice for KeyboardDevice {
    type InputState = KeyboardInputState;

    fn read_raw(&mut self, blocking: bool) -> CruilResult<&[u8]> {
        self.device.set_blocking_mode(blocking)?;
        let read = self.device.read(&mut self.buffer)?;
        let report = &self.buffer[..read];
        Ok(report)
    }

    fn read(&mut self, blocking: bool) -> CruilResult<KeyboardInputState> {
        let report = self.read_raw(blocking)?;
        let report_length = report.len();
        let overflow = report.get(2) == Some(&KEY_ERR_OVF);

        if report_length == 0 || overflow {
            // Gracefully handle overflow and no response by returning last known state
            return Ok(KeyboardInputState {
                currently_pressed: self.last_pressed.clone(),
                overflow,
                ..Default::default()
            });
        }

        if report_length < 2 {
            return Err(CruilError::ProtocolViolation(
                ProtocolViolation::ResponseTooShort(report_length),
            ));
        }

        let currently_pressed = KeySet {
            modifiers: Modifiers::from_bits_retain(report[0]),
            keys: report[2..]
                .iter()
                .filter(|&&k| k != KEY_NONE)
                .filter_map(|v| v.try_into().inspect_err(|e| println!("{e}, ignoring")).ok())
                .collect(),
        };

        let just_pressed = currently_pressed.difference(&self.last_pressed);
        let just_released = self.last_pressed.difference(&currently_pressed);

        self.last_pressed = currently_pressed.clone();

        Ok(KeyboardInputState {
            overflow,
            currently_pressed,
            just_pressed,
            just_released,
        })
    }
}
