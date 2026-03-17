use crate::CruilResult;
use hidapi::{DeviceInfo, HidApi, MAX_REPORT_DESCRIPTOR_SIZE};

/// An enum containing cruil's built-in supported device kinds, [`Keyboard`] and [`Mouse`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeviceKind {
    Keyboard,
    Mouse,
}
use DeviceKind::*;
use hidparser::ReportField;

impl DeviceKind {
    #[doc = include_str!("device_kind_from_info.md")]
    #[cfg(target_os = "macos")]
    pub fn from_info(info: &DeviceInfo) -> CruilResult<DeviceKind> {
        match (info.usage_page(), info.usage()) {
            (1, 6) => Ok(Keyboard),
            (1, 2) => Ok(Mouse),
            (p, u) => Err(crate::CruilError::UnsupportedDeviceKind(p, u)),
        }
    }

    #[doc = include_str!("device_kind_from_info.md")]
    #[cfg(not(target_os = "macos"))]
    pub fn from_info(info: &DeviceInfo) -> CruilResult<DeviceKind> {
        // TEMPORARY HACK: Remove this when refactoring (hopefully soon)
        // This API will hopefully become private anyway and the type of device will be provided by the library
        // Also something something multiple backends
        // This must not end up in a release in this state!
        let hid = HidApi::new()?;
        let device = info.open_device(&hid)?;
        let mut buffer = [0; MAX_REPORT_DESCRIPTOR_SIZE];
        let report_descriptor_size = device.get_report_descriptor(&mut buffer)?;
        let report_descriptor = &buffer[..report_descriptor_size];
        let report_descriptor = hidparser::parse_report_descriptor(report_descriptor).expect(
            "Error in temporary code that should not be relied upon or end up in a release",
        );
        // This is a dirty dirty evil hack that does not produce accurate results
        // I need to test if any of this works at all though
        report_descriptor
            .input_reports
            .iter()
            .find_map(|r| {
                r.fields.iter().find_map(|f| {
                    if let ReportField::Variable(v) = f {
                        v.member_of
                            .iter()
                            .find_map(|c| match (c.usage.page(), c.usage.id()) {
                                (1, 6) => Some(Keyboard),
                                (1, 2) => Some(Mouse),
                                _ => None,
                            })
                    } else {
                        None
                    }
                })
            })
            .ok_or(crate::CruilError::UnsupportedDeviceKind(0, 0))
    }
}
