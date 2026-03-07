use cruil::*;

fn main() {
    let mut cruil = Cruil::new().unwrap();

    let mut keyboard = cruil
        .open_first_available_with(|d| {
            d.product_string()
                .is_some_and(|v| v.starts_with("Microsoft"))
                && matches!(DeviceKind::from_info(d), Ok(DeviceKind::Keyboard))
        })
        .expect("Could not find/open Microsoft keyboard")
        .keyboard()
        .expect("Microsoft keyboard was actually a mouse");
    println!("Opened Microsoft keyboard");

    loop {
        let report = keyboard.read(true).unwrap();
        println!(
            "Overflow: {}, Currently pressed: {}, Just pressed: {}, Just released: {}",
            report.overflow, report.currently_pressed, report.just_pressed, report.just_released
        );
    }
}
