use cruil::*;

fn main() {
    let mut input_reader = Cruil::new().unwrap();

    if !input_reader.open_first_available_with(|d| {
        d.product_string()
            .is_some_and(|v| v.starts_with("Microsoft"))
            && DeviceKind::from_info(d) == Some(DeviceKind::Keyboard)
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
