use super::KeySet;

/// The input state of a keyboard.
///
/// This includes:
/// - Overflow state ([`overflow`](Self::overflow))
/// - The current state of keyboard keys ([`currently_pressed`](Self::currently_pressed))
/// - Key states compared to the previous state ([`just_pressed`](Self::just_pressed), [`just_released`](Self::just_released))
#[derive(Debug, Clone, Default)]
pub struct KeyboardInputState {
    /// Whether the keyboard is reporting overflow. (too many keys pressed)
    ///
    /// This may not be accurate in [ghost reports](crate::ReadableDevice::read).
    ///
    /// If this is `true`, the previous state is assumed to still uphold,
    /// so [`just_pressed`](Self::just_pressed) and [`just_released`](Self::just_released) will be empty.
    pub overflow: bool,
    /// The set of keyboard keys that are currently pressed.
    pub currently_pressed: KeySet,
    /// The set of keyboard keys that were just pressed. (That were not pressed during the previous state)
    pub just_pressed: KeySet,
    /// The set of keyboard keys that were just released. (That were pressed during the previous state)
    pub just_released: KeySet,
}
