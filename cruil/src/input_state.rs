use crate::keyboard::*;
use crate::mouse::*;

pub enum InputState {
    Keyboard(KeyboardInputState),
    Mouse(MouseInputState),
}
