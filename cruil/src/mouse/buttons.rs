use bitflags::bitflags;
use std::fmt::Display;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
    pub struct MouseButtons: u8 {
        const LEFT     = 0b00000001;
        const RIGHT    = 0b00000010;
        const MIDDLE   = 0b00000100;
        const MOUSE4   = 0b00001000;
        const MOUSE5   = 0b00010000;
        const UNKNOWN6 = 0b00100000;
        const UNKNOWN7 = 0b01000000;
        const UNKNOWN8 = 0b10000000;
    }
}

impl MouseButtons {
    pub const MODIFIER_NAME_MAP: [(MouseButtons, &str); 8] = [
        (Self::LEFT, "Left"),
        (Self::RIGHT, "Right"),
        (Self::MIDDLE, "Middle"),
        (Self::MOUSE4, "Mouse4"),
        (Self::MOUSE5, "Mouse5"),
        (Self::UNKNOWN6, "Unknown6"),
        (Self::UNKNOWN7, "Unknown7"),
        (Self::UNKNOWN8, "Unknown8"),
    ];
}

impl Display for MouseButtons {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            return write!(f, "None");
        }
        let keys: Vec<&str> = Self::MODIFIER_NAME_MAP
            .iter()
            .filter_map(|(key, name)| self.intersects(*key).then_some(*name))
            .collect();
        write!(f, "{}", keys.join("+"))
    }
}
