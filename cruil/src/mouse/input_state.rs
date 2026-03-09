use super::MouseButtons;

#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct MouseInputState {
    pub delta_x: i16,
    pub delta_y: i16,
    pub currently_pressed: MouseButtons,
    pub just_pressed: MouseButtons,
    pub just_released: MouseButtons,
    pub delta_wheel: i8,
}
