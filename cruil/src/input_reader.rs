#[cfg(target_os = "macos")]
use crate::CruilError;
use crate::{CruilResult, DeviceKind, KeyboardDevice, KeyboardInputState};
use hidapi::{DeviceInfo, HidApi, HidError, HidResult};

pub struct InputReader {
    hid: HidApi,
    devices: Vec<KeyboardDevice>,
}

impl InputReader {
    pub fn new() -> HidResult<Self> {
        let hid = HidApi::new()?;

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

        Ok(InputReader {
            hid,
            devices: vec![],
        })
    }

    pub fn open_first_available_with(&mut self, condition: impl Fn(&DeviceInfo) -> bool) -> bool {
        match self.open_with(&condition) {
            Ok(()) => true,
            Err(true) => {
                // Try again
                self.open_with(&condition).is_ok()
            }
            Err(false) => false,
        }
    }

    fn open_with(&mut self, condition: &impl Fn(&DeviceInfo) -> bool) -> Result<(), bool> {
        for device in self.hid.device_list() {
            if !condition(device) {
                continue;
            }

            match self.attempt_open_device(device) {
                Ok(device) => {
                    self.devices.push(device);
                    return Ok(());
                }
                #[cfg(target_os = "macos")]
                Err(CruilError::HidApi(HidError::HidApiError { message }))
                    if message.contains("0xE0005000") =>
                {
                    println!("USB pipe stalled");
                    return Err(true);
                }
                Err(e) => {
                    println!("Unknown error, trying next: {e}");
                    continue;
                }
            }
        }

        Err(false)
    }

    fn attempt_open_device(&self, info: &DeviceInfo) -> CruilResult<KeyboardDevice> {
        let mut device = KeyboardDevice::new(info.open_device(&self.hid)?);
        println!("Performing read check...");
        _ = device.read_raw()?;
        Ok(device)
    }

    pub fn read_all_raw(&mut self) -> CruilResult<Vec<&[u8]>> {
        self.devices
            .iter_mut()
            .map(KeyboardDevice::read_raw)
            .collect()
    }

    pub fn read_all(&mut self) -> CruilResult<Vec<KeyboardInputState>> {
        self.devices.iter_mut().map(KeyboardDevice::read).collect()
    }
}
