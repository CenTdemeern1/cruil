use crate::{
    CruilError, CruilResult, KeySet, KeyboardInputState, ProtocolViolation, ReadableDevice,
    keys::{
        Key, Modifiers,
        raw::{KEY_ERR_OVF, KEY_NONE},
    },
};
use hidapi::HidDevice;

/// Allegedly, full-speed HID packets are still at most 64 bytes.
const MAX_HID_PACKET_SIZE: usize = 64;

/// A keyboard device. Represents a HID keyboard.
///
/// This is usually equivalent to a single physical keyboard, but any singular physical USB device could pretend to be any number of keyboards.
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

impl KeyboardDevice {
    /// Returns whether the given key is pressed.
    pub fn is_key_pressed(&self, key: &Key) -> bool {
        self.last_pressed.contains_key(key)
    }

    /// Returns whether the given modifier key is pressed.
    pub fn is_modifier_pressed(&self, modifiers: Modifiers) -> bool {
        self.last_pressed.contains_modifier(modifiers)
    }

    /// Returns whether all the keys in the given set are pressed.
    pub fn is_set_pressed(&self, set: KeySet) -> bool {
        set.is_subset(&self.last_pressed)
    }
}

impl ReadableDevice for KeyboardDevice {
    type State = KeyboardInputState;

    fn read_raw(&mut self, blocking: bool) -> CruilResult<&[u8]> {
        self.device.set_blocking_mode(blocking)?;
        let read = self.device.read(&mut self.buffer)?;
        let report = &self.buffer[..read];
        Ok(report)
    }

    fn read(&mut self, blocking: bool) -> CruilResult<Self::State> {
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
