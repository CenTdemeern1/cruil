use crate::{CruilResult, DeviceKind, InputState, KeyboardDevice, ReadableDevice};
use hidapi::HidDevice;

pub enum InputDevice {
    Keyboard(KeyboardDevice),
    Mouse,
}
use InputDevice::*;

impl InputDevice {
    pub(crate) fn new(device: HidDevice, kind: DeviceKind) -> Self {
        match kind {
            DeviceKind::Keyboard => Keyboard(KeyboardDevice::new(device)),
            DeviceKind::Mouse => todo!(),
        }
    }

    pub fn keyboard(self) -> Option<KeyboardDevice> {
        match self {
            Keyboard(d) => Some(d),
            _ => None,
        }
    }
}

impl ReadableDevice for InputDevice {
    type InputState = InputState;

    fn read_raw(&mut self, blocking: bool) -> CruilResult<&[u8]> {
        match self {
            InputDevice::Keyboard(keyboard_device) => keyboard_device.read_raw(blocking),
            InputDevice::Mouse => todo!(),
        }
    }

    fn read(&mut self, blocking: bool) -> CruilResult<Self::InputState> {
        Ok(match self {
            InputDevice::Keyboard(keyboard_device) => {
                InputState::Keyboard(keyboard_device.read(blocking)?)
            }
            InputDevice::Mouse => todo!(),
        })
    }
}
