use crate::device::CruilDevice;
use cruil::ThreadedReader;
use godot::prelude::*;

/// Singleton for Cruil's Godot bindings.
#[derive(GodotClass)]
#[class(init, singleton, base = Object)]
pub struct Cruil {
    cruil: Option<cruil::Cruil>,

    base: Base<Object>,
}

impl Cruil {
    fn push_uninitialized_error() {
        godot_error!("Attempt to call Cruil function while Cruil is uninitialized");
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
    pub fn open_all(&mut self) -> Vec<Gd<CruilDevice>> {
        let Some(cruil) = &mut self.cruil else {
            Self::push_uninitialized_error();
            return vec![];
        };
        cruil.refresh().unwrap();
        cruil
            .open_all()
            .into_iter()
            .map(ThreadedReader::start)
            .map(CruilDevice::new)
            .collect()
    }
}
