use cruil::{InputDevice, ThreadedReader};
use godot::{
    global::{Error, push_error},
    prelude::*,
};
use std::collections::HashMap;

/// Singleton for Cruil's Godot bindings.
#[derive(GodotClass)]
#[class(init, singleton, base = Object)]
pub struct Cruil {
    cruil: Option<cruil::Cruil>,
    devices: HashMap<usize, ThreadedReader<InputDevice>>,
    unused_id: usize,

    base: Base<Object>,
}

impl Cruil {
    fn push_uninitialized_error() {
        godot_error!("Attempt to call Cruil function while Cruil is uninitialized");
    }

    fn postfix_increment(n: &mut usize) -> usize {
        let old_n = *n;
        *n += 1;
        old_n
    }
}

#[godot_api]
impl Cruil {
    /// Initialize Cruil.
    ///
    /// If Cruil fails to initialize, the error is pushed to the debugger and this function returns false.
    #[func]
    pub fn init(&mut self) -> bool {
        if self.cruil.is_some() {
            return true;
        }

        match cruil::Cruil::new() {
            Ok(cruil) => {
                self.cruil = Some(cruil);
                true
            }
            Err(e) => {
                godot_error!("Cruil failed to initialize: {e}");
                false
            }
        }
    }

    #[func]
    pub fn open_all(&mut self) {
        let Some(cruil) = &mut self.cruil else {
            Self::push_uninitialized_error();
            return;
        };
        cruil.refresh().unwrap();
        self.devices
            .extend(cruil.open_all().into_iter().map(|device| {
                (
                    Self::postfix_increment(&mut self.unused_id),
                    ThreadedReader::start(device),
                )
            }));
    }
}
