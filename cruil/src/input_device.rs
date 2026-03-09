use crate::keyboard::*;
use crate::mouse::*;
use crate::*;
use hidapi::HidDevice;

/// An opened input device, which is either a [`Keyboard`] or a [`Mouse`]. (The two types of devices Cruil has built-in support for)
///
/// # Usage
/// The `InputDevice` can either be [`read`](Self::read) directly to get an [`InputState`],
/// a union enum of [`KeyboardInputState`] and [`MouseInputState`],
/// or it can be narrowed down to a [`KeyboardDevice`] or [`MouseDevice`] by matching or
/// using the [`keyboard`](Self::keyboard) and [`mouse`](Self::mouse) helper methods.
///
/// For more information on reading input from devices, see [`ReadableDevice`].
pub enum InputDevice {
    /// The opened device is a keyboard.
    Keyboard(KeyboardDevice),
    /// The opened device is a mouse.
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

    /// Turns this `InputDevice` into an <code>[Option]<[KeyboardDevice]></code>.
    ///
    /// Similarly to [`Result::ok`], this will consume `self` and discard the inner value if it was a [`Mouse`].
    pub fn keyboard(self) -> Option<KeyboardDevice> {
        match self {
            Keyboard(d) => Some(d),
            _ => None,
        }
    }

    /// Turns this `InputDevice` into an <code>[Option]<[MouseDevice]></code>.
    ///
    /// Similarly to [`Result::ok`], this will consume `self` and discard the inner value if it was a [`Keyboard`].
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
