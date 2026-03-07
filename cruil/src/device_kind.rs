use hidapi::DeviceInfo;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum DeviceKind {
    Keyboard,
    Mouse,
}
use DeviceKind::*;

impl DeviceKind {
    #[doc = include_str!("device_kind_from_info.md")]
    #[cfg(any(target_os = "windows", target_os = "macos"))]
    pub fn from_info(info: &DeviceInfo) -> Option<DeviceKind> {
        match (info.usage_page(), info.usage()) {
            (1, 6) => Some(Keyboard),
            (1, 2) => Some(Mouse),
            _ => None,
        }
    }

    #[doc = include_str!("device_kind_from_info.md")]
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    pub fn from_info(info: &DeviceInfo) -> DeviceKind {
        todo!("Report descriptor parsing is not yet implemented")
    }
}
