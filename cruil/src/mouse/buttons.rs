use bitflags::bitflags;
use std::fmt::Display;

bitflags! {
    /// A set of mouse buttons, as bits.
    ///
    /// This is a [`bitflags`](::bitflags) struct.
    /// You can compare this set with its constants to see whether one or more buttons are (or aren't) pressed.
    ///
    /// For example:
    /// ```
    /// # use cruil::mouse::*;
    /// // Let's pretend we got these from a MouseInputState
    /// let buttons = MouseButtons::LEFT | MouseButtons::RIGHT;
    ///
    /// assert!(buttons.contains(MouseButtons::LEFT));
    /// assert_eq!(
    ///     buttons.intersection(MouseButtons::MIDDLE | MouseButtons::RIGHT),
    ///     MouseButtons::RIGHT
    /// );
    /// ```
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
    pub struct MouseButtons: u8 {
        /// Left mouse button.
        const LEFT     = 0b00000001;
        /// Right mouse button.
        const RIGHT    = 0b00000010;
        /// Middle mouse button. (Clicking the scroll wheel)
        const MIDDLE   = 0b00000100;
        /// The back button, also known as Mouse4. (Often located on the side of a mouse)
        const BACK     = 0b00001000;
        /// The forward button, also known as Mouse5. (Often located on the side of a mouse)
        const FORWARD  = 0b00010000;
        const UNKNOWN6 = 0b00100000;
        const UNKNOWN7 = 0b01000000;
        const UNKNOWN8 = 0b10000000;
    }
}

impl MouseButtons {
    pub(crate) const BUTTON_NAME_MAP: [(MouseButtons, &str); 8] = [
        (Self::LEFT, "Left"),
        (Self::RIGHT, "Right"),
        (Self::MIDDLE, "Middle"),
        (Self::BACK, "Back"),
        (Self::FORWARD, "Forward"),
        (Self::UNKNOWN6, "Unknown6"),
        (Self::UNKNOWN7, "Unknown7"),
        (Self::UNKNOWN8, "Unknown8"),
    ];
}

impl Display for MouseButtons {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            return write!(f, "None");
        }
        let keys: Vec<&str> = Self::BUTTON_NAME_MAP
            .iter()
            .filter_map(|(key, name)| self.intersects(*key).then_some(*name))
            .collect();
        write!(f, "{}", keys.join("+"))
    }
}
