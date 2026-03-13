use cruil::{InputDevice, ThreadedReader};
use godot::{
    classes::{Engine, Input, InputEvent, InputEventKey, MainLoop},
    prelude::*,
    register::ConnectHandle,
};
use std::{mem::swap, num::NonZeroI32};

use crate::event::CruilEvent;

/// An open Cruil input device.
#[derive(GodotClass)]
#[class(no_init, base = Object)]
pub struct CruilDevice {
    device: ThreadedReader<InputDevice>,
    id: i32,
    connection: Option<ConnectHandle>,

    base: Base<Object>,
}

impl CruilDevice {
    pub fn new(device: ThreadedReader<InputDevice>) -> Gd<Self> {
        Gd::from_init_fn(|base| CruilDevice {
            device,
            id: 0,
            connection: None,
            base,
        })
    }

    fn fatal(&self) {
        godot_warn!("CruilDevice is freeing self to prevent memory leak after fatal error.");
        self.to_gd().free();
    }

    fn disconnect(&mut self) {
        if let Some(connection) = self.connection.take() {
            connection.disconnect();
        }
    }

    fn process_frame(&mut self) {
        if self.id == 0 {
            // This should theoretically never happen
            self.disconnect();
            return;
        }
        let input = Input::singleton();
        while let Some(event) = self.get_input_event() {
            for event in event.to_events() {
                input.parse_input_event(event);
            }
        }
    }
}

#[godot_api]
impl CruilDevice {
    /// Maps this device to a specific device ID in input events. ([member InputEvent.device])
    ///
    /// Once a device has an ID mapped, its input events will start being propagated through Godot's input system,
    /// and you will be able to read them with functions like [method Node._input] and [method Node._unhandled_input].
    ///
    /// An ID of `0` will disable this functionality.
    /// (device `0` in keyboard/mouse input events is Godot's default value,
    /// and all keyboard and mouse events handled by Godot use it,
    /// so you wouldn't be able to distinguish different devices.)
    #[func]
    pub fn map_event_id(&mut self, id: i32) {
        let Some(main_loop) = Engine::singleton().get_main_loop() else {
            godot_error!("No mainloop to attach to.");
            self.fatal();
            return;
        };
        let Ok(scene_tree) = main_loop.try_cast::<SceneTree>() else {
            godot_error!("Abnormal mainloop (not SceneTree).");
            self.fatal();
            return;
        };
        self.disconnect();
        self.id = id;
        self.connection = Some(
            scene_tree
                .signals()
                .process_frame()
                .connect_other(self, Self::process_frame),
        );
    }

    /// Try to get a new input event from this device.
    ///
    /// This function should be called repeatedly
    #[func]
    pub fn get_input_event(&self) -> Option<Gd<CruilEvent>> {
        self.device
            .iter()
            .filter_map(Result::ok)
            .next()
            .map(CruilEvent::new)
    }
}
