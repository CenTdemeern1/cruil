use cruil::keyboard::*;
use cruil::*;

fn main() -> CruilResult<()> {
    let cruil = Cruil::new()?;

    let mut devices: Vec<KeyboardDevice> = cruil
        .open_all_with(|info| matches!(DeviceKind::from_info(info), Ok(DeviceKind::Keyboard)))
        .into_iter()
        .map(|device| device.keyboard())
        .collect::<Option<Vec<_>>>()
        .unwrap();

    // The first keyboard to press a key becomes player one!
    println!("Press any key to join!");
    let player_one = 'player_select: loop {
        for (index, device) in devices.iter_mut().enumerate() {
            if let Some(report) = device.try_read()?
                && report.just_pressed.any()
            {
                // First key press!
                break 'player_select index;
            }
        }
    };

    // Let's drop all other devices and get ownership over the player
    let player_one = devices.into_iter().nth(player_one).unwrap();

    // Let's start reading player one's input in the background
    let player_one = ThreadedReader::start(player_one);

    // Game loop...
    loop {
        let events: Vec<KeyboardInputState> = player_one.iter().collect::<CruilResult<_>>()?;
        for event in events {
            // Process game input...
            if event.currently_pressed.any() {
                println!("Player 1 is pressing {}", event.currently_pressed);
            }
        }

        // Process the game...
    }
}
