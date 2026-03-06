use hidapi::DeviceInfo;

#[derive(Debug, PartialEq, Eq)]
pub enum DeviceKind {
    Other,
    Keyboard,
    Mouse,
}

impl DeviceKind {
    #[cfg(any(target_os = "windows", target_os = "macos"))]
    pub fn from_info(info: &DeviceInfo) -> DeviceKind {
        use DeviceKind::*;
        match (info.usage_page(), info.usage()) {
            (1, 2) => Mouse,
            (1, 6) => Keyboard,
            _ => Other,
        }
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    pub fn from_info(info: &DeviceInfo) -> DeviceKind {
        todo!("Report descriptor parsing is not yet implemented")
    }
}
