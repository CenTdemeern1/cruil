use crate::keyboard::*;
use crate::mouse::*;
use crate::*;
use hidapi::HidDevice;

pub enum InputDevice {
    Keyboard(KeyboardDevice),
    Mouse(MouseDevice),
}
use InputDevice::*;

impl InputDevice {
    pub(crate) fn new(device: HidDevice, kind: DeviceKind) -> Self {
        match kind {
            DeviceKind::Keyboard => Keyboard(KeyboardDevice::new(device)),
            DeviceKind::Mouse => Mouse(MouseDevice::new(device)),
        }
    }

    pub fn keyboard(self) -> Option<KeyboardDevice> {
        match self {
            Keyboard(d) => Some(d),
            _ => None,
        }
    }

    pub fn mouse(self) -> Option<MouseDevice> {
        match self {
            Mouse(d) => Some(d),
            _ => None,
        }
    }
}

impl ReadableDevice for InputDevice {
    type State = InputState;

    fn read_raw(&mut self, blocking: bool) -> CruilResult<&[u8]> {
        match self {
            Keyboard(keyboard_device) => keyboard_device.read_raw(blocking),
            Mouse(mouse_device) => mouse_device.read_raw(blocking),
        }
    }

    fn read(&mut self, blocking: bool) -> CruilResult<Self::State> {
        Ok(match self {
            Keyboard(keyboard_device) => InputState::Keyboard(keyboard_device.read(blocking)?),
            Mouse(mouse_device) => InputState::Mouse(mouse_device.read(blocking)?),
        })
    }
}
