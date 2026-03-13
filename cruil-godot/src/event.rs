use std::ops::BitOr;

use cruil::{
    InputState,
    keyboard::{KeySet, KeyboardInputState, keys::Modifiers},
    mouse::MouseButtons,
};
use godot::{
    classes::{InputEvent, InputEventKey, InputEventMouseButton, InputEventMouseMotion},
    global::{MouseButton, MouseButtonMask},
    prelude::*,
};

use crate::key::{modifiers_to_godot_key, to_godot_key};

#[derive(GodotClass)]
#[class(no_init, base = RefCounted)]
pub struct CruilEvent {
    state: InputState,

    base: Base<RefCounted>,
}

impl CruilEvent {
    const MOUSE_BUTTON_MAP: [(MouseButtons, MouseButtonMask); 5] = [
        (MouseButtons::LEFT, MouseButtonMask::LEFT),
        (MouseButtons::RIGHT, MouseButtonMask::RIGHT),
        (MouseButtons::MIDDLE, MouseButtonMask::MIDDLE),
        (MouseButtons::BACK, MouseButtonMask::MB_XBUTTON1),
        (MouseButtons::FORWARD, MouseButtonMask::MB_XBUTTON2),
    ];

    pub fn new(state: InputState) -> Gd<Self> {
        Gd::from_init_fn(|base| CruilEvent { state, base })
    }

    fn process_keyboard<'s>(
        set: &'s KeySet,
        pressed: bool,
        alt_pressed: bool,
        ctrl_pressed: bool,
        meta_pressed: bool,
        shift_pressed: bool,
    ) -> impl Iterator<Item = Gd<InputEvent>> + use<'s> {
        modifiers_to_godot_key(set.modifiers)
            .chain(set.keys.iter().copied().map(to_godot_key))
            .map(move |(key, location)| {
                let mut event = InputEventKey::new_gd();
                event.set_alt_pressed(alt_pressed);
                event.set_ctrl_pressed(ctrl_pressed);
                event.set_meta_pressed(meta_pressed);
                event.set_shift_pressed(shift_pressed);
                event.set_echo(false);
                event.set_key_label(key);
                event.set_keycode(key);
                event.set_location(location);
                event.set_physical_keycode(key);
                event.set_pressed(pressed);
                event.upcast()
            })
    }

    fn any_of_modifiers_pressed(state: &KeyboardInputState, modifiers: Modifiers) -> bool {
        state.currently_pressed.modifiers.intersects(modifiers)
    }

    fn convert_mouse_button(
        state: MouseButtons,
        button: MouseButtons,
        to: MouseButtonMask,
    ) -> MouseButtonMask {
        state.contains(button).then_some(to).unwrap_or_default()
    }

    fn convert_mouse_buttons(state: MouseButtons) -> MouseButtonMask {
        Self::MOUSE_BUTTON_MAP
            .iter()
            .copied()
            .map(|(buttons, mask)| Self::convert_mouse_button(state, buttons, mask))
            .reduce(BitOr::bitor)
            .unwrap_or_default()
    }

    fn process_mouse_buttons(
        buttons: MouseButtons,
        button_mask: MouseButtonMask,
        pressed: bool,
    ) -> impl Iterator<Item = Gd<InputEvent>> {
        buttons
            .into_iter()
            .filter_map(|b| {
                Some(match b {
                    MouseButtons::LEFT => MouseButton::LEFT,
                    MouseButtons::RIGHT => MouseButton::RIGHT,
                    MouseButtons::MIDDLE => MouseButton::MIDDLE,
                    MouseButtons::BACK => MouseButton::XBUTTON1,
                    MouseButtons::FORWARD => MouseButton::XBUTTON2,
                    _ => return None,
                })
            })
            .map(move |button_index| {
                let mut event = InputEventMouseButton::new_gd();
                event.set_button_mask(button_mask);
                event.set_button_index(button_index);
                event.set_pressed(pressed);
                event.upcast()
            })
    }

    fn process_scroll_wheel(
        wheel: i8,
        button_mask: MouseButtonMask,
    ) -> impl Iterator<Item = Gd<InputEvent>> {
        let direction = if wheel.is_negative() {
            MouseButton::WHEEL_DOWN
        } else {
            MouseButton::WHEEL_UP
        };
        let amount = wheel.unsigned_abs();
        (0..amount).flat_map(move |_| {
            let mut event_press = InputEventMouseButton::new_gd();
            event_press.set_button_mask(button_mask);
            event_press.set_button_index(direction);
            event_press.set_pressed(true);
            let mut event_release = InputEventMouseButton::new_gd();
            event_release.set_button_mask(button_mask);
            event_release.set_button_index(direction);
            event_release.set_pressed(false);
            [event_press.upcast(), event_release.upcast()]
        })
    }
}

#[godot_api]
impl CruilEvent {
    #[func]
    pub fn to_events(&self) -> Vec<Gd<InputEvent>> {
        match &self.state {
            InputState::Keyboard(state) => {
                let alt_pressed = Self::any_of_modifiers_pressed(&state, Modifiers::ALT);
                let ctrl_pressed = Self::any_of_modifiers_pressed(&state, Modifiers::CTRL);
                let meta_pressed = Self::any_of_modifiers_pressed(&state, Modifiers::SUPER);
                let shift_pressed = Self::any_of_modifiers_pressed(&state, Modifiers::SHIFT);
                Self::process_keyboard(
                    &state.just_released,
                    false,
                    alt_pressed,
                    ctrl_pressed,
                    meta_pressed,
                    shift_pressed,
                )
                .chain(Self::process_keyboard(
                    &state.just_pressed,
                    true,
                    alt_pressed,
                    ctrl_pressed,
                    meta_pressed,
                    shift_pressed,
                ))
                .collect()
            }
            InputState::Mouse(state) => {
                let button_mask = Self::convert_mouse_buttons(state.currently_pressed);
                ((state.delta_x, state.delta_y) != (0, 0))
                    .then(|| {
                        let mut event = InputEventMouseMotion::new_gd();
                        event.set_button_mask(button_mask);
                        event.set_relative(Vector2::new(state.delta_x as _, state.delta_y as _));
                        event.upcast()
                    })
                    .into_iter()
                    .chain(Self::process_mouse_buttons(
                        state.just_released,
                        button_mask,
                        false,
                    ))
                    .chain(Self::process_mouse_buttons(
                        state.just_pressed,
                        button_mask,
                        true,
                    ))
                    .chain(Self::process_scroll_wheel(state.delta_wheel, button_mask))
                    .collect()
            }
        }
    }
}
