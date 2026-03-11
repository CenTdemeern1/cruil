#[cfg(doc)]
use crate::InputDevice;
use crate::keyboard::*;
use crate::mouse::*;

/// An input state obtained by [`read`](InputDevice#method.read)ing an [`InputDevice`].
///
/// This `InputState` may either be a [`Keyboard`] or a [`Mouse`]
/// depending on what kind of device the [`InputDevice`] it was read from is.
///
/// To get a [`KeyboardInputState`] or [`MouseInputState`] without having to `match` this enum,
/// consider narrowing down the type of device by matching the [`InputDevice`] instead,
/// which will give you a [`KeyboardDevice`] or [`MouseDevice`] which, when read,
/// will give you the appropriate kind of input state directly.
pub enum InputState {
    /// An input state coming from a keyboard.
    Keyboard(KeyboardInputState),
    /// An input state coming from a mouse.
    Mouse(MouseInputState),
}
use InputState::*;

impl InputState {
    /// Turns this `InputState` into an <code>[Option]<[KeyboardInputState]></code>.
    ///
    /// Similarly to [`Result::ok`], this will consume `self` and discard the inner value if it was a [`Mouse`].
    pub fn keyboard(self) -> Option<KeyboardInputState> {
        match self {
            Keyboard(s) => Some(s),
            _ => None,
        }
    }

    /// Turns this `InputState` into an <code>[Option]<[MouseInputState]></code>.
    ///
    /// Similarly to [`Result::ok`], this will consume `self` and discard the inner value if it was a [`Keyboard`].
    pub fn mouse(self) -> Option<MouseInputState> {
        match self {
            Mouse(s) => Some(s),
            _ => None,
        }
    }

    /// Returns whether this `InputState` is a [`Keyboard`].
    pub fn is_keyboard(&self) -> bool {
        matches!(self, Keyboard(_))
    }

    /// Returns whether this `InputState` is a [`Mouse`].
    pub fn is_mouse(&self) -> bool {
        matches!(self, Mouse(_))
    }
}
