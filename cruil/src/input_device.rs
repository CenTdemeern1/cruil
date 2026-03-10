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

    /// Returns whether this `InputDevice` is a [`Keyboard`].
    pub fn is_keyboard(&self) -> bool {
        matches!(self, Keyboard(_))
    }

    /// Returns whether this `InputDevice` is a [`Mouse`].
    pub fn is_mouse(&self) -> bool {
        matches!(self, Mouse(_))
    }

    /// This is a hack that's mainly present to help perform the read check.
    pub(crate) fn read_internal_buffer(&mut self, blocking: bool) -> CruilResult<usize> {
        match self {
            Keyboard(keyboard_device) => keyboard_device.read_internal_buffer(blocking),
            Mouse(mouse_device) => mouse_device.read_internal_buffer(blocking),
        }
    }
}

impl ReadableDevice for InputDevice {
    type State = InputState;

    /// The output from `<InputDevice as ReadableDevice>::read_raw` does not have an easy way to tell
    /// what kind of device the data was read from, or how to parse it.
    ///
    /// Consider using [`is_keyboard`](Self::is_keyboard) or [`is_mouse`](Self::is_mouse)
    /// if you need to use `read_raw` on an `InputDevice` that hasn't been narrowed down yet.
    fn read_raw(&self, buffer: &mut [u8], blocking: bool) -> CruilResult<usize> {
        match self {
            Keyboard(keyboard_device) => keyboard_device.read_raw(buffer, blocking),
            Mouse(mouse_device) => mouse_device.read_raw(buffer, blocking),
        }
    }

    fn try_read(&mut self) -> CruilResult<Option<Self::State>> {
        match self {
            Keyboard(k_device) => k_device.try_read().map(|o| o.map(InputState::Keyboard)),
            Mouse(m_device) => m_device.try_read().map(|o| o.map(InputState::Mouse)),
        }
    }

    fn read(&mut self, blocking: bool) -> CruilResult<Self::State> {
        Ok(match self {
            Keyboard(keyboard_device) => InputState::Keyboard(keyboard_device.read(blocking)?),
            Mouse(mouse_device) => InputState::Mouse(mouse_device.read(blocking)?),
        })
    }
}
