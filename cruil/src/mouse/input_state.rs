use super::MouseButtons;

/// The input state of a mouse.
///
/// This includes:
/// - Movement data ([`delta_x`](Self::delta_x), [`delta_y`](Self::delta_y))
/// - Button states ([`currently_pressed`](Self::currently_pressed))
/// - Button states compared to the previous state ([`just_pressed`](Self::just_pressed), [`just_released`](Self::just_released))
/// - Scrolling data ([`delta_wheel`](Self::delta_wheel))
#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct MouseInputState {
    /// The horizontal movement of the mouse.
    pub delta_x: i16,
    /// The vertical movement of the mouse.
    pub delta_y: i16,
    /// The current state of the mouse buttons.
    pub currently_pressed: MouseButtons,
    /// A list of mouse buttons that were just pressed. (That were not pressed during the previous state)
    pub just_pressed: MouseButtons,
    /// A list of mouse buttons that were just released. (That were pressed during the previous state)
    pub just_released: MouseButtons,
    /// The movement of the scroll wheel.
    pub delta_wheel: i8,
}
