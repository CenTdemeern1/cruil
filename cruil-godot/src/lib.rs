use godot::prelude::*;

mod cruil;
mod device;
mod event;
mod key;

struct CruilExtension;

#[gdextension]
unsafe impl ExtensionLibrary for CruilExtension {}
