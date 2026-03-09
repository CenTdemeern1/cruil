use crate::keyboard::keys::raw::{KEY_ERR_POST_FAIL, KEY_ERR_UNDEFINED};
use hidapi::HidError;
use thiserror::Error;

/// Cruil's [`Result`] type.
pub type CruilResult<T> = Result<T, CruilError>;

/// An error that indicates that something went wrong with the keyboard device, presumably a hardware fault.
///
/// Not quite ["printer on fire"](https://en.wikipedia.org/wiki/Lp0_on_fire), but I wish the best of luck to any users that encounter this.
#[derive(Debug, Error, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum KeyboardError {
    /// The keyboard failed to POST.
    #[error("The keyboard failed to POST")]
    POSTFail = KEY_ERR_POST_FAIL,
    /// The keyboard ran into an unknown error.
    #[error("The keyboard ran into an unknown error")]
    Undefined = KEY_ERR_UNDEFINED,
}

/// An error that indicates that the device sent an invalid response.
#[derive(Debug, Error, PartialEq, Eq, Hash)]
pub enum ProtocolViolation {
    /// The device sent a response that was too short.
    #[error("The device's response was too short ({0})")]
    ResponseTooShort(usize),
}

/// Cruil's all-encompassing error type.
// TODO: Try to get rid of the HidApi variant and other variants that can be separated
#[derive(Debug, Error)]
pub enum CruilError {
    /// Something went wrong with the keyboard device.
    // TODO: Currently unimplemented
    #[error("Keyboard error: {0}")]
    KeyboardError(KeyboardError),
    /// The device sent an invalid response.
    #[error("Protocol violation: {0}")]
    ProtocolViolation(ProtocolViolation),
    /// This kind of device is unsupported and cannot be opened by cruil.
    #[error("Unsupported device ({0:#X}, {1:#X})")]
    UnsupportedDeviceKind(u16, u16),
    /// The USB pipe stalled.
    #[error("USB pipe stalled")]
    PipeStalled,
    /// Internal unhandled error.
    #[error("hidapi error: {0}")]
    HidApi(#[from] HidError),
}

/// An error that indicates that the given key code sent by the device is unknown and could not be parsed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Error)]
#[error("Unrecognized key {0:#X}")]
pub struct UnrecognizedKey(pub u8);
