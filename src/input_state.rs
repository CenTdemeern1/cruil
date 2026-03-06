use crate::KeySet;

#[derive(Debug, Clone, Default)]
pub struct InputState {
    pub overflow: bool,
    pub currently_pressed: KeySet,
    pub just_pressed: KeySet,
    pub just_released: KeySet,
}
