use super::keys::*;
use super::*;
use crate::*;
use hidapi::HidDevice;

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

    /// Returns whether the given key is pressed.
    pub fn is_key_pressed(&self, key: &Key) -> bool {
        self.last_pressed.contains_key(key)
    }

    /// Returns whether the given modifier keys are pressed.
    pub fn are_modifiers_pressed(&self, modifiers: Modifiers) -> bool {
        self.last_pressed.contains_modifiers(modifiers)
    }

    /// Returns whether all the keys in the given set are pressed.
    pub fn is_set_pressed(&self, set: KeySet) -> bool {
        set.is_subset(&self.last_pressed)
    }

    pub(crate) fn read_internal_buffer(&mut self, blocking: bool) -> CruilResult<usize> {
        self.device.set_blocking_mode(blocking)?;
        Ok(self.device.read(&mut self.buffer)?)
    }

    fn parse_internal_buffer(&mut self, report_length: usize) -> CruilResult<KeyboardInputState> {
        let report = &self.buffer[..report_length];
        let overflow = report.get(2) == Some(&raw::KEY_ERR_OVF);

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
                .filter(|&&k| k != raw::KEY_NONE)
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

impl ReadableDevice for KeyboardDevice {
    type State = KeyboardInputState;

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

impl IntoIterator for KeyboardDevice {
    type IntoIter = OwnedReadableDeviceIter<Self>;
    type Item = CruilResult<KeyboardInputState>;

    #[doc(alias = "owned_iter")]
    fn into_iter(self) -> Self::IntoIter {
        self.owned_iter()
    }
}
