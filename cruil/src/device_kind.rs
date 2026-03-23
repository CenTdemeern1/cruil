use crate::CruilResult;
use hidapi::DeviceInfo;

/// An enum containing cruil's built-in supported device kinds, [`Keyboard`] and [`Mouse`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeviceKind {
    Keyboard,
    Mouse,
}
use DeviceKind::*;

impl DeviceKind {
    pub(crate) const KEYBOARD_USAGE: (u16, u16) = (1, 6);
    pub(crate) const MOUSE_USAGE: (u16, u16) = (1, 2);

    /// Tries to figure out what kind of device the [`DeviceInfo`] is describing.
    /// This is achieved through looking at its Usage info.
    ///
    /// If the type of device is unsupported (not a [Mouse] or [Keyboard]),
    /// this function returns [`None`].
    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    pub fn from_info(info: &DeviceInfo) -> CruilResult<DeviceKind> {
        match (info.usage_page(), info.usage()) {
            Self::KEYBOARD_USAGE => Ok(Keyboard),
            Self::MOUSE_USAGE => Ok(Mouse),
            (p, u) => Err(crate::CruilError::UnsupportedDeviceKind(p, u)),
        }
    }

    /// Returns (page, usage)
    pub(crate) fn to_hid_usage(self) -> (u16, u16) {
        match self {
            Keyboard => Self::KEYBOARD_USAGE,
            Mouse => Self::MOUSE_USAGE,
        }
    }
}
