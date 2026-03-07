use crate::keys::raw::{KEY_ERR_POST_FAIL, KEY_ERR_UNDEFINED};
use hidapi::HidError;
use thiserror::Error;

pub type CruilResult<T> = Result<T, CruilError>;

#[derive(Debug, Error, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum KeyboardError {
    #[error("The keyboard failed to POST")]
    POSTFail = KEY_ERR_POST_FAIL,
    #[error("The keyboard ran into an unknown error")]
    Undefined = KEY_ERR_UNDEFINED,
}

#[derive(Debug, Error, PartialEq, Eq, Hash)]
pub enum ProtocolViolation {
    #[error("The device's response was too short ({0})")]
    ResponseTooShort(usize),
}

#[derive(Debug, Error)]
pub enum CruilError {
    #[error("Keyboard error: {0}")]
    KeyboardError(KeyboardError),
    #[error("Protocol violation: {0}")]
    ProtocolViolation(ProtocolViolation),
    #[error("Unsupported device ({0:#X}, {1:#X})")]
    UnsupportedDeviceKind(u16, u16),
    #[error("hidapi error: {0}")]
    HidApi(#[from] HidError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Error)]
#[error("Unrecognized key {0:#X}")]
pub struct UnrecognizedKey(pub u8);
