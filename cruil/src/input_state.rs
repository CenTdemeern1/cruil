use crate::KeyboardInputState;

pub enum InputState {
    Keyboard(KeyboardInputState),
    Mouse,
}
