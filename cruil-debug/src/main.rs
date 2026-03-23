#![allow(unused)]

use cruil::keyboard::*;
use cruil::mouse::*;
use cruil::*;

fn main() {
    let mut cruil = Cruil::new().unwrap();

    let keyboard = cruil
        .open_first_available_with(|d| {
            d.product_string()
                .is_some_and(|v| v.starts_with("Microsoft"))
                && matches!(DeviceKind::from_info(d), Ok(DeviceKind::Keyboard))
        })
        .expect("Could not find/open Microsoft keyboard");
    println!("Opened Microsoft keyboard");

    // let mouse = cruil
    //     .open_first_available_with(|d| {
    //         d.product_string()
    //             .is_some_and(|v| v.starts_with("MX Master"))
    //             && matches!(DeviceKind::from_info(d), Ok(DeviceKind::Mouse))
    //     })
    //     .expect("Could not find/open Logi mouse");
    // println!("Opened Logi mouse");

    // let mouse = cruil
    //     .open_first_available_with(|d| {
    //         d.product_string()
    //             .is_some_and(|v| v.starts_with("USB OPTICAL MOUSE"))
    //             && matches!(DeviceKind::from_info(d), Ok(DeviceKind::Mouse))
    //     })
    //     .expect("Could not find/open trash mouse");
    // println!("Opened trash mouse");

    test_blocking(keyboard)
}

fn test_blocking(mut device: InputDevice) -> ! {
    loop {
        let report = device.read(true).unwrap();
        print_report(report);
    }
}

fn test_nonblocking(keyboard: InputDevice) -> ! {
    let reader = ThreadedReader::start(keyboard);
    let mut iter = reader.iter();

    loop {
        if let Some(Ok(report)) = iter.next() {
            print_report(report);
        }
    }
}

fn print_report(report: InputState) {
    match report {
        InputState::Keyboard(KeyboardInputState {
            overflow,
            currently_pressed,
            just_pressed,
            just_released,
        }) => {
            println!(
                "Overflow: {overflow}, Currently pressed: {currently_pressed}, Just pressed: {just_pressed}, Just released: {just_released}"
            );
        }
        InputState::Mouse(MouseInputState {
            delta_x,
            delta_y,
            currently_pressed,
            just_pressed,
            just_released,
            delta_wheel,
        }) => {
            println!(
                "Move delta: ({delta_x}, {delta_y}), Currently pressed: {currently_pressed}, Just pressed: {just_pressed}, Just released: {just_released}, Delta wheel: {delta_wheel}"
            );
        }
    }
}
