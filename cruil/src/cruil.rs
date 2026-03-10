use crate::{CruilError, CruilResult, DeviceKind, InputDevice};
use hidapi::{DeviceInfo, HidApi, HidError};

/// Cruil's main struct. Get started here!
pub struct Cruil {
    hid: HidApi,
}

impl Cruil {
    /// Create a new instance of Cruil.
    ///
    /// Each instance has its own copy of [`HidApi`] with its own device cache.
    ///
    /// Aside from allocating a new device cache,
    /// creating multiple copies of Cruil is fairly cheap.
    pub fn new() -> CruilResult<Self> {
        let hid = HidApi::new()?;

        #[cfg(feature = "debug_logging")]
        for device in hid.device_list() {
            let name = device
                .product_string()
                .map(|v| format!("{v:?}"))
                .unwrap_or_else(|| "<unreadable name>".to_string());
            println!(
                "Device {name} connected via {:?}, usage ({:#X}, {:#X}) = {:?}",
                device.bus_type(),
                device.usage_page(),
                device.usage(),
                DeviceKind::from_info(device)
            );
        }

        Ok(Cruil { hid })
    }

    /// Refresh the device list.
    pub fn refresh(&mut self) -> CruilResult<()> {
        Ok(self.hid.refresh_devices()?)
    }

    /// Returns an iterator over the list of supported devices.
    pub fn devices(&self) -> impl Iterator<Item = &DeviceInfo> {
        self.hid
            .device_list()
            .filter(|&info| DeviceKind::from_info(info).is_ok())
    }

    /// Returns the first device that met the given condition and successfully opened.
    ///
    /// All errors of failed attempts are returned if no device was successfully opened.
    /// If no device met the given condition, this vec is empty and unallocated.
    ///
    /// ```no_run
    /// # use cruil::*;
    /// let mut cruil = Cruil::new().unwrap();
    /// // Opens the first available keyboard
    /// let keyboard = cruil
    ///     .open_first_available_with(|device_info| {
    ///         matches!(DeviceKind::from_info(device_info), Ok(DeviceKind::Keyboard))
    ///     })
    ///     .unwrap();
    /// ```
    pub fn open_first_available_with(
        &self,
        mut condition: impl FnMut(&DeviceInfo) -> bool,
    ) -> Result<InputDevice, Vec<CruilError>> {
        let mut errors: Vec<CruilError> = vec![];

        for device in self.hid.device_list() {
            if !condition(device) {
                continue;
            }

            match self.attempt_open_device_with_retry(device) {
                Ok(device) => {
                    return Ok(device);
                }
                Err(e) => {
                    #[cfg(feature = "debug_logging")]
                    println!("Unknown error, trying next: {e}");
                    errors.push(e);
                    continue;
                }
            }
        }

        Err(errors)
    }

    /// Opens all keyboards and mice.
    ///
    /// Returns all devices that successfully opened, ignoring errors.
    pub fn open_all_with(
        &self,
        mut condition: impl FnMut(&DeviceInfo) -> bool,
    ) -> Vec<InputDevice> {
        self.hid
            .device_list()
            .filter(|&info| condition(info))
            .filter_map(|info| self.attempt_open_device_with_retry(info).ok())
            .collect()
    }

    /// Opens all keyboards and mice.
    ///
    /// Returns all devices that successfully opened, ignoring errors.
    pub fn open_all(&self) -> Vec<InputDevice> {
        self.open_all_with(|_| true)
    }

    fn attempt_open_device_with_retry(&self, info: &DeviceInfo) -> CruilResult<InputDevice> {
        let kind = DeviceKind::from_info(info)?;
        match self.attempt_open_device(info, kind) {
            Err(CruilError::PipeStalled) => {
                // Try again
                #[cfg(feature = "debug_logging")]
                println!("USB pipe stalled");
                self.attempt_open_device(info, kind)
            }
            v => v,
        }
    }

    fn attempt_open_device(&self, info: &DeviceInfo, kind: DeviceKind) -> CruilResult<InputDevice> {
        let mut device = InputDevice::new(info.open_device(&self.hid)?, kind);

        // Stupid hack to handle this error because it returns a *string* for some reason... ugh
        // TODO: Also implement this for other operating systems
        #[cfg(target_os = "macos")]
        {
            #[cfg(feature = "debug_logging")]
            println!("Performing read check...");
            if let Err(CruilError::HidApi(HidError::HidApiError { message })) =
                device.read_internal_buffer(false)
                && message.contains("0xE0005000")
            {
                return Err(CruilError::PipeStalled);
            }
            #[cfg(feature = "debug_logging")]
            println!("Read check OK");
        }

        Ok(device)
    }
}
