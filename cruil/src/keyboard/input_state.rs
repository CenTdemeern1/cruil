use super::KeySet;

#[derive(Debug, Clone, Default)]
pub struct KeyboardInputState {
    pub overflow: bool,
    pub currently_pressed: KeySet,
    pub just_pressed: KeySet,
    pub just_released: KeySet,
}
