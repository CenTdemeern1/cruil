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
    /// Tries to figure out what kind of device the [`DeviceInfo`] is describing.
    /// This is achieved through looking at its Usage info.
    ///
    /// If the type of device is unsupported (not a [Mouse] or [Keyboard]),
    /// this function returns [`None`].
    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    pub fn from_info(info: &DeviceInfo) -> CruilResult<DeviceKind> {
        match (info.usage_page(), info.usage()) {
            (1, 6) => Ok(Keyboard),
            (1, 2) => Ok(Mouse),
            (p, u) => Err(crate::CruilError::UnsupportedDeviceKind(p, u)),
        }
    }
}
