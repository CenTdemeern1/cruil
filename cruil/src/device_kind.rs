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
    #[doc = include_str!("device_kind_from_info.md")]
    #[cfg(any(target_os = "windows", target_os = "macos"))]
    pub fn from_info(info: &DeviceInfo) -> CruilResult<DeviceKind> {
        match (info.usage_page(), info.usage()) {
            (1, 6) => Ok(Keyboard),
            (1, 2) => Ok(Mouse),
            (p, u) => Err(crate::CruilError::UnsupportedDeviceKind(p, u)),
        }
    }

    #[doc = include_str!("device_kind_from_info.md")]
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    pub fn from_info(info: &DeviceInfo) -> CruilResult<DeviceKind> {
        todo!("Report descriptor parsing is not yet implemented")
    }
}
