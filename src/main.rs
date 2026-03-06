mod device_kind;
pub use device_kind::*;
mod input_device;
pub use input_device::*;
mod input_reader;
pub use input_reader::*;
mod input_state;
pub use input_state::*;
mod key_set;
pub use key_set::*;
mod error;
pub use error::*;
pub mod keys;

fn main() {
    let mut input_reader = InputReader::new().unwrap();

    if !input_reader.open_first_available_with(|d| {
        d.product_string()
            .is_some_and(|v| v.starts_with("Microsoft"))
            && DeviceKind::from_info(d) == DeviceKind::Keyboard
    }) {
        panic!("Could not find/open Microsoft keyboard");
    }

    println!("Opened Microsoft keyboard");

    // if !input_reader.open_first_available_with(|d| {
    //     d.product_string().is_some_and(|v| v.starts_with("Wooting"))
    //         && DeviceKind::from_info(d) == DeviceKind::Keyboard
    // }) {
    //     panic!("Could not find/open Wooting keyboard");
    // }

    // println!("Opened Wooting keyboard");
    loop {
        for (device, report) in input_reader.read_all().unwrap().into_iter().enumerate() {
            // println!("Device {device}: {report:?}");
            println!(
                "Device {device}: Overflow: {}, Currently pressed: {}, Just pressed: {}, Just released: {}",
                report.overflow,
                report.currently_pressed,
                report.just_pressed,
                report.just_released
            );
        }
    }
}
