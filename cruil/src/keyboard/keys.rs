//! All the different keyboard keys
//!
//! Based on <https://gist.github.com/MightyPork/6da26e382a7ad91b5496ee55fdc73db2> with heavy modifications
//! and <https://www.usb.org/sites/default/files/hut1_7.pdf>

use std::fmt::Display;

use crate::UnrecognizedKey;
use bitflags::bitflags;
use raw::*;

bitflags! {
    /// A set of modifier keys, as bits.
    ///
    /// This is a [`bitflags`](::bitflags) struct.
    /// You can compare this set with its constants to see whether one or more modifiers are (or aren't) pressed.
    ///
    /// For example:
    /// ```
    /// # use cruil::keyboard::keys::*;
    /// // Let's pretend we got these from a KeyboardInputState
    /// let buttons = Modifiers::LCTRL | Modifiers::RSHIFT;
    ///
    /// // Checks whether left ctrl is pressed
    /// assert!(buttons.contains(Modifiers::LCTRL));
    ///
    /// // This checks whether *any* ctrl key is pressed.
    /// // Modifiers::CTRL is LCTRL | RCTRL, so
    /// // Modifiers::contains() would check if *both* are pressed.
    /// assert!(buttons.intersects(Modifiers::CTRL));
    ///
    /// assert_eq!(
    ///     // Which shift key is pressed?
    ///     buttons.intersection(Modifiers::SHIFT),
    ///     // The right one
    ///     Modifiers::RSHIFT
    /// );
    /// ```
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
    pub struct Modifiers: u8 {
        /// Left control.
        const LCTRL  = 0b00000001;
        /// Left shift.
        const LSHIFT = 0b00000010;
        /// Left alt.
        const LALT   = 0b00000100;
        /// Left super/meta/gui/win.
        const LSUPER = 0b00001000;
        /// Right control.
        const RCTRL  = 0b00010000;
        /// Right shift.
        const RSHIFT = 0b00100000;
        /// Right alt.
        const RALT   = 0b01000000;
        /// Right super/meta/gui/win.
        const RSUPER = 0b10000000;

        /// Both control keys.
        const CTRL = Self::LCTRL.bits() | Self::RCTRL.bits();
        /// Both shift keys.
        const SHIFT = Self::LSHIFT.bits() | Self::RSHIFT.bits();
        /// Both alt keys.
        const ALT = Self::LALT.bits() | Self::RALT.bits();
        /// Both super/meta/gui/win keys.
        const SUPER = Self::LSUPER.bits() | Self::RSUPER.bits();
    }
}

impl Modifiers {
    pub(crate) const MODIFIER_NAME_MAP: [(Modifiers, &str); 8] = [
        (Self::LCTRL, "LControl"),
        (Self::LSHIFT, "LShift"),
        (Self::LALT, "LAlt"),
        (Self::LSUPER, "LSuper"),
        (Self::RCTRL, "RControl"),
        (Self::RSHIFT, "RShift"),
        (Self::RALT, "RAlt"),
        (Self::RSUPER, "RSuper"),
    ];
}

impl Display for Modifiers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keys: Vec<&str> = Self::MODIFIER_NAME_MAP
            .iter()
            .filter_map(|(key, name)| self.intersects(*key).then_some(*name))
            .collect();
        write!(f, "{}", keys.join("+"))
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Key {
    /// Keyboard a and A
    A = KEY_A,
    /// Keyboard b and B
    B = KEY_B,
    /// Keyboard c and C
    C = KEY_C,
    /// Keyboard d and D
    D = KEY_D,
    /// Keyboard e and E
    E = KEY_E,
    /// Keyboard f and F
    F = KEY_F,
    /// Keyboard g and G
    G = KEY_G,
    /// Keyboard h and H
    H = KEY_H,
    /// Keyboard i and I
    I = KEY_I,
    /// Keyboard j and J
    J = KEY_J,
    /// Keyboard k and K
    K = KEY_K,
    /// Keyboard l and L
    L = KEY_L,
    /// Keyboard m and M
    M = KEY_M,
    /// Keyboard n and N
    N = KEY_N,
    /// Keyboard o and O
    O = KEY_O,
    /// Keyboard p and P
    P = KEY_P,
    /// Keyboard q and Q
    Q = KEY_Q,
    /// Keyboard r and R
    R = KEY_R,
    /// Keyboard s and S
    S = KEY_S,
    /// Keyboard t and T
    T = KEY_T,
    /// Keyboard u and U
    U = KEY_U,
    /// Keyboard v and V
    V = KEY_V,
    /// Keyboard w and W
    W = KEY_W,
    /// Keyboard x and X
    X = KEY_X,
    /// Keyboard y and Y
    Y = KEY_Y,
    /// Keyboard z and Z
    Z = KEY_Z,

    /// Keyboard 1 and !
    Num1 = KEY_1,
    /// Keyboard 2 and @
    Num2 = KEY_2,
    /// Keyboard 3 and #
    Num3 = KEY_3,
    /// Keyboard 4 and $
    Num4 = KEY_4,
    /// Keyboard 5 and %
    Num5 = KEY_5,
    /// Keyboard 6 and ^
    Num6 = KEY_6,
    /// Keyboard 7 and &
    Num7 = KEY_7,
    /// Keyboard 8 and *
    Num8 = KEY_8,
    /// Keyboard 9 and (
    Num9 = KEY_9,
    /// Keyboard 0 and )
    Num0 = KEY_0,

    /// Keyboard Return (ENTER)
    Enter = KEY_ENTER,
    /// Keyboard ESCAPE
    Esc = KEY_ESC,
    /// Keyboard DELETE (Backspace)
    Backspace = KEY_BACKSPACE,
    /// Keyboard Tab
    Tab = KEY_TAB,
    /// Keyboard Spacebar
    Space = KEY_SPACE,
    /// Keyboard - and _
    Minus = KEY_MINUS,
    /// Keyboard = and +
    Equal = KEY_EQUAL,
    /// Keyboard [ and {
    LeftBrace = KEY_LEFTBRACE,
    /// Keyboard ] and }
    RightBrace = KEY_RIGHTBRACE,
    /// Keyboard \ and |
    Backslash = KEY_BACKSLASH,
    /// Keyboard Non-US # and ~
    HashTildeNonUS = KEY_HASHTILDE_NON_US,
    /// Keyboard ; and :
    Semicolon = KEY_SEMICOLON,
    /// Keyboard ' and "
    Apostrophe = KEY_APOSTROPHE,
    /// Keyboard ` and ~
    Grave = KEY_GRAVE,
    /// Keyboard , and <
    Comma = KEY_COMMA,
    /// Keyboard . and >
    Dot = KEY_DOT,
    /// Keyboard / and ?
    Slash = KEY_SLASH,
    /// Keyboard Caps Lock
    CapsLock = KEY_CAPSLOCK,

    /// Keyboard F1
    F1 = KEY_F1,
    /// Keyboard F2
    F2 = KEY_F2,
    /// Keyboard F3
    F3 = KEY_F3,
    /// Keyboard F4
    F4 = KEY_F4,
    /// Keyboard F5
    F5 = KEY_F5,
    /// Keyboard F6
    F6 = KEY_F6,
    /// Keyboard F7
    F7 = KEY_F7,
    /// Keyboard F8
    F8 = KEY_F8,
    /// Keyboard F9
    F9 = KEY_F9,
    /// Keyboard F10
    F10 = KEY_F10,
    /// Keyboard F11
    F11 = KEY_F11,
    /// Keyboard F12
    F12 = KEY_F12,

    /// Keyboard Print Screen
    PrintScreen = KEY_PRINTSCREEN,
    /// Keyboard Scroll Lock
    ScrollLock = KEY_SCROLLLOCK,
    /// Keyboard Pause
    Pause = KEY_PAUSE,
    /// Keyboard Insert
    Insert = KEY_INSERT,
    /// Keyboard Home
    Home = KEY_HOME,
    /// Keyboard Page Up
    PageUp = KEY_PAGEUP,
    /// Keyboard Delete Forward
    Delete = KEY_DELETE,
    /// Keyboard End
    End = KEY_END,
    /// Keyboard Page Down
    PageDown = KEY_PAGEDOWN,
    /// Keyboard Right Arrow
    Right = KEY_RIGHT,
    /// Keyboard Left Arrow
    Left = KEY_LEFT,
    /// Keyboard Down Arrow
    Down = KEY_DOWN,
    /// Keyboard Up Arrow
    Up = KEY_UP,

    /// Keyboard Num Lock and Clear
    NumLock = KEY_NUMLOCK,
    /// Keypad /
    KpSlash = KEY_KPSLASH,
    /// Keypad *
    KpAsterisk = KEY_KPASTERISK,
    /// Keypad -
    KpMinus = KEY_KPMINUS,
    /// Keypad +
    KpPlus = KEY_KPPLUS,
    /// Keypad ENTER
    KpEnter = KEY_KPENTER,
    /// Keypad 1 and End
    Kp1 = KEY_KP1,
    /// Keypad 2 and Down Arrow
    Kp2 = KEY_KP2,
    /// Keypad 3 and PageDn
    Kp3 = KEY_KP3,
    /// Keypad 4 and Left Arrow
    Kp4 = KEY_KP4,
    /// Keypad 5
    Kp5 = KEY_KP5,
    /// Keypad 6 and Right Arrow
    Kp6 = KEY_KP6,
    /// Keypad 7 and Home
    Kp7 = KEY_KP7,
    /// Keypad 8 and Up Arrow
    Kp8 = KEY_KP8,
    /// Keypad 9 and Page Up
    Kp9 = KEY_KP9,
    /// Keypad 0 and Insert
    Kp0 = KEY_KP0,
    /// Keypad . and Delete
    KpDot = KEY_KPDOT,

    /// Keyboard Non-US \ and |
    BackslashNonUS = KEY_BACKSLASH_NON_US,
    /// Keyboard Application
    Compose = KEY_COMPOSE,
    /// Keyboard Power
    Power = KEY_POWER,
    /// Keypad =
    KpEqual = KEY_KPEQUAL,

    /// Keyboard F13
    F13 = KEY_F13,
    /// Keyboard F14
    F14 = KEY_F14,
    /// Keyboard F15
    F15 = KEY_F15,
    /// Keyboard F16
    F16 = KEY_F16,
    /// Keyboard F17
    F17 = KEY_F17,
    /// Keyboard F18
    F18 = KEY_F18,
    /// Keyboard F19
    F19 = KEY_F19,
    /// Keyboard F20
    F20 = KEY_F20,
    /// Keyboard F21
    F21 = KEY_F21,
    /// Keyboard F22
    F22 = KEY_F22,
    /// Keyboard F23
    F23 = KEY_F23,
    /// Keyboard F24
    F24 = KEY_F24,

    /// Keyboard Execute
    Open = KEY_OPEN,
    /// Keyboard Help
    Help = KEY_HELP,
    /// Keyboard Menu
    Props = KEY_PROPS,
    /// Keyboard Select
    Front = KEY_FRONT,
    /// Keyboard Stop
    Stop = KEY_STOP,
    /// Keyboard Again
    Again = KEY_AGAIN,
    /// Keyboard Undo
    Undo = KEY_UNDO,
    /// Keyboard Cut
    Cut = KEY_CUT,
    /// Keyboard Copy
    Copy = KEY_COPY,
    /// Keyboard Paste
    Paste = KEY_PASTE,
    /// Keyboard Find
    Find = KEY_FIND,
    /// Keyboard Mute
    Mute = KEY_MUTE,
    /// Keyboard Volume Up
    VolumeUp = KEY_VOLUMEUP,
    /// Keyboard Volume Down
    VolumeDown = KEY_VOLUMEDOWN,
    // 0x82  Keyboard Locking Caps Lock
    // 0x83  Keyboard Locking Num Lock
    // 0x84  Keyboard Locking Scroll Lock
    /// Keypad Comma
    KpComma = KEY_KPCOMMA,
    // 0x86  Keypad Equal Sign
    /// Keyboard International1, allocated to the ろ (ro) key
    Ro = KEY_RO,
    /// Keyboard International2, allocated to the カタカナ/ひらがな (hiragana/katakana toggle) key
    KatakanaHiragana = KEY_KATAKANAHIRAGANA,
    /// Keyboard International3, allocated to the ¥ (yen) key
    Yen = KEY_YEN,
    /// Keyboard International4, allocated to the 変換 (henkan, convert from kana to kanji) key
    Henkan = KEY_HENKAN,
    /// Keyboard International5, allocated to the 無変換 (muhenkan, don't convert from kana to kanji) key
    Muhenkan = KEY_MUHENKAN,
    /// Keyboard International6, allocated to the Japanese keypad comma key found on PC98 keyboards (?)
    KpJPComma = KEY_KPJPCOMMA,
    // 0x8d  Keyboard International7
    // 0x8e  Keyboard International8
    // 0x8f  Keyboard International9
    /// Keyboard LANG1, allocated to the 한/영 (han/yeong, hangul/english toggle) key
    Hangul = KEY_HANGUL,
    /// Keyboard LANG2, allocated to the 한자 (hanja, convert from hangul to hanja) key
    Hanja = KEY_HANJA,
    /// Keyboard LANG3, allocated to the dedicated katakana key on Japanese word-processing keyboards
    /// (citation needed? i can't find anything that doesn't word-for-word copy the same phrasing)
    Katakana = KEY_KATAKANA,
    /// Keyboard LANG4, allocated to the dedicated hiragana key on Japanese word-processing keyboards
    /// (citation needed? i can't find anything that doesn't word-for-word copy the same phrasing)
    Hiragana = KEY_HIRAGANA,
    /// Keyboard LANG5, allocated to the 半角/全角 (hankaku/zenkaku, halfwidth/fullwidth toggle) key
    HankakuZenkaku = KEY_HANKAKUZENKAKU,
    // 0x95  Keyboard LANG6
    // 0x96  Keyboard LANG7
    // 0x97  Keyboard LANG8
    // 0x98  Keyboard LANG9
    // 0x99  Keyboard Alternate Erase
    // 0x9a  Keyboard SysReq/Attention
    // 0x9b  Keyboard Cancel
    // 0x9c  Keyboard Clear
    // 0x9d  Keyboard Prior
    // 0x9e  Keyboard Return
    // 0x9f  Keyboard Separator
    // 0xa0  Keyboard Out
    // 0xa1  Keyboard Oper
    // 0xa2  Keyboard Clear/Again
    // 0xa3  Keyboard CrSel/Props
    // 0xa4  Keyboard ExSel

    // 0xb0  Keypad 00
    // 0xb1  Keypad 000
    // 0xb2  Thousands Separator
    // 0xb3  Decimal Separator
    // 0xb4  Currency Unit
    // 0xb5  Currency Sub-unit
    /// Keypad (
    KpLeftParen = KEY_KPLEFTPAREN,
    /// Keypad )
    KpRightParen = KEY_KPRIGHTPAREN,
    // 0xb8  Keypad {
    // 0xb9  Keypad }
    // 0xba  Keypad Tab
    // 0xbb  Keypad Backspace
    // 0xbc  Keypad A
    // 0xbd  Keypad B
    // 0xbe  Keypad C
    // 0xbf  Keypad D
    // 0xc0  Keypad E
    // 0xc1  Keypad F
    // 0xc2  Keypad XOR
    // 0xc3  Keypad ^
    // 0xc4  Keypad %
    // 0xc5  Keypad <
    // 0xc6  Keypad >
    // 0xc7  Keypad &
    // 0xc8  Keypad &&
    // 0xc9  Keypad |
    // 0xca  Keypad ||
    // 0xcb  Keypad :
    // 0xcc  Keypad #
    // 0xcd  Keypad Space
    // 0xce  Keypad @
    // 0xcf  Keypad !
    // 0xd0  Keypad Memory Store
    // 0xd1  Keypad Memory Recall
    // 0xd2  Keypad Memory Clear
    // 0xd3  Keypad Memory Add
    // 0xd4  Keypad Memory Subtract
    // 0xd5  Keypad Memory Multiply
    // 0xd6  Keypad Memory Divide
    // 0xd7  Keypad +/-
    // 0xd8  Keypad Clear
    // 0xd9  Keypad Clear Entry
    // 0xda  Keypad Binary
    // 0xdb  Keypad Octal
    // 0xdc  Keypad Decimal
    // 0xdd  Keypad Hexadecimal
    /// Keyboard Left Control
    ///
    /// You probably want to check [`Modifiers`] instead!
    LeftCtrl = KEY_LEFTCTRL,
    /// Keyboard Left Shift
    ///
    /// You probably want to check [`Modifiers`] instead!
    LeftShift = KEY_LEFTSHIFT,
    /// Keyboard Left Alt
    ///
    /// You probably want to check [`Modifiers`] instead!
    LeftAlt = KEY_LEFTALT,
    /// Keyboard Left Super
    ///
    /// You probably want to check [`Modifiers`] instead!
    LeftSuper = KEY_LEFTSUPER,
    /// Keyboard Right Control
    ///
    /// You probably want to check [`Modifiers`] instead!
    RightCtrl = KEY_RIGHTCTRL,
    /// Keyboard Right Shift
    ///
    /// You probably want to check [`Modifiers`] instead!
    RightShift = KEY_RIGHTSHIFT,
    /// Keyboard Right Alt
    ///
    /// You probably want to check [`Modifiers`] instead!
    RightAlt = KEY_RIGHTALT,
    /// Keyboard Right Super
    ///
    /// You probably want to check [`Modifiers`] instead!
    RightSuper = KEY_RIGHTSUPER,

    MediaPlaypause = KEY_MEDIA_PLAYPAUSE,
    MediaStopcd = KEY_MEDIA_STOPCD,
    MediaPrevioussong = KEY_MEDIA_PREVIOUSSONG,
    MediaNextsong = KEY_MEDIA_NEXTSONG,
    MediaEjectcd = KEY_MEDIA_EJECTCD,
    MediaVolumeup = KEY_MEDIA_VOLUMEUP,
    MediaVolumedown = KEY_MEDIA_VOLUMEDOWN,
    MediaMute = KEY_MEDIA_MUTE,
    MediaWww = KEY_MEDIA_WWW,
    MediaBack = KEY_MEDIA_BACK,
    MediaForward = KEY_MEDIA_FORWARD,
    MediaStop = KEY_MEDIA_STOP,
    MediaFind = KEY_MEDIA_FIND,
    MediaScrollup = KEY_MEDIA_SCROLLUP,
    MediaScrolldown = KEY_MEDIA_SCROLLDOWN,
    MediaEdit = KEY_MEDIA_EDIT,
    MediaSleep = KEY_MEDIA_SLEEP,
    MediaCoffee = KEY_MEDIA_COFFEE,
    MediaRefresh = KEY_MEDIA_REFRESH,
    MediaCalc = KEY_MEDIA_CALC,
}

impl TryFrom<&u8> for Key {
    type Error = UnrecognizedKey;

    fn try_from(&value: &u8) -> Result<Self, Self::Error> {
        use Key::*;
        Ok(match value {
            KEY_A => A,
            KEY_B => B,
            KEY_C => C,
            KEY_D => D,
            KEY_E => E,
            KEY_F => F,
            KEY_G => G,
            KEY_H => H,
            KEY_I => I,
            KEY_J => J,
            KEY_K => K,
            KEY_L => L,
            KEY_M => M,
            KEY_N => N,
            KEY_O => O,
            KEY_P => P,
            KEY_Q => Q,
            KEY_R => R,
            KEY_S => S,
            KEY_T => T,
            KEY_U => U,
            KEY_V => V,
            KEY_W => W,
            KEY_X => X,
            KEY_Y => Y,
            KEY_Z => Z,
            KEY_1 => Num1,
            KEY_2 => Num2,
            KEY_3 => Num3,
            KEY_4 => Num4,
            KEY_5 => Num5,
            KEY_6 => Num6,
            KEY_7 => Num7,
            KEY_8 => Num8,
            KEY_9 => Num9,
            KEY_0 => Num0,
            KEY_ENTER => Enter,
            KEY_ESC => Esc,
            KEY_BACKSPACE => Backspace,
            KEY_TAB => Tab,
            KEY_SPACE => Space,
            KEY_MINUS => Minus,
            KEY_EQUAL => Equal,
            KEY_LEFTBRACE => LeftBrace,
            KEY_RIGHTBRACE => RightBrace,
            KEY_BACKSLASH => Backslash,
            KEY_HASHTILDE_NON_US => HashTildeNonUS,
            KEY_SEMICOLON => Semicolon,
            KEY_APOSTROPHE => Apostrophe,
            KEY_GRAVE => Grave,
            KEY_COMMA => Comma,
            KEY_DOT => Dot,
            KEY_SLASH => Slash,
            KEY_CAPSLOCK => CapsLock,
            KEY_F1 => F1,
            KEY_F2 => F2,
            KEY_F3 => F3,
            KEY_F4 => F4,
            KEY_F5 => F5,
            KEY_F6 => F6,
            KEY_F7 => F7,
            KEY_F8 => F8,
            KEY_F9 => F9,
            KEY_F10 => F10,
            KEY_F11 => F11,
            KEY_F12 => F12,
            KEY_PRINTSCREEN => PrintScreen,
            KEY_SCROLLLOCK => ScrollLock,
            KEY_PAUSE => Pause,
            KEY_INSERT => Insert,
            KEY_HOME => Home,
            KEY_PAGEUP => PageUp,
            KEY_DELETE => Delete,
            KEY_END => End,
            KEY_PAGEDOWN => PageDown,
            KEY_RIGHT => Right,
            KEY_LEFT => Left,
            KEY_DOWN => Down,
            KEY_UP => Up,
            KEY_NUMLOCK => NumLock,
            KEY_KPSLASH => KpSlash,
            KEY_KPASTERISK => KpAsterisk,
            KEY_KPMINUS => KpMinus,
            KEY_KPPLUS => KpPlus,
            KEY_KPENTER => KpEnter,
            KEY_KP1 => Kp1,
            KEY_KP2 => Kp2,
            KEY_KP3 => Kp3,
            KEY_KP4 => Kp4,
            KEY_KP5 => Kp5,
            KEY_KP6 => Kp6,
            KEY_KP7 => Kp7,
            KEY_KP8 => Kp8,
            KEY_KP9 => Kp9,
            KEY_KP0 => Kp0,
            KEY_KPDOT => KpDot,
            KEY_BACKSLASH_NON_US => BackslashNonUS,
            KEY_COMPOSE => Compose,
            KEY_POWER => Power,
            KEY_KPEQUAL => KpEqual,
            KEY_F13 => F13,
            KEY_F14 => F14,
            KEY_F15 => F15,
            KEY_F16 => F16,
            KEY_F17 => F17,
            KEY_F18 => F18,
            KEY_F19 => F19,
            KEY_F20 => F20,
            KEY_F21 => F21,
            KEY_F22 => F22,
            KEY_F23 => F23,
            KEY_F24 => F24,
            KEY_OPEN => Open,
            KEY_HELP => Help,
            KEY_PROPS => Props,
            KEY_FRONT => Front,
            KEY_STOP => Stop,
            KEY_AGAIN => Again,
            KEY_UNDO => Undo,
            KEY_CUT => Cut,
            KEY_COPY => Copy,
            KEY_PASTE => Paste,
            KEY_FIND => Find,
            KEY_MUTE => Mute,
            KEY_VOLUMEUP => VolumeUp,
            KEY_VOLUMEDOWN => VolumeDown,
            KEY_KPCOMMA => KpComma,
            KEY_RO => Ro,
            KEY_KATAKANAHIRAGANA => KatakanaHiragana,
            KEY_YEN => Yen,
            KEY_HENKAN => Henkan,
            KEY_MUHENKAN => Muhenkan,
            KEY_KPJPCOMMA => KpJPComma,
            KEY_HANGUL => Hangul,
            KEY_HANJA => Hanja,
            KEY_KATAKANA => Katakana,
            KEY_HIRAGANA => Hiragana,
            KEY_HANKAKUZENKAKU => HankakuZenkaku,
            KEY_KPLEFTPAREN => KpLeftParen,
            KEY_KPRIGHTPAREN => KpRightParen,
            KEY_LEFTCTRL => LeftCtrl,
            KEY_LEFTSHIFT => LeftShift,
            KEY_LEFTALT => LeftAlt,
            KEY_LEFTSUPER => LeftSuper,
            KEY_RIGHTCTRL => RightCtrl,
            KEY_RIGHTSHIFT => RightShift,
            KEY_RIGHTALT => RightAlt,
            KEY_RIGHTSUPER => RightSuper,
            KEY_MEDIA_PLAYPAUSE => MediaPlaypause,
            KEY_MEDIA_STOPCD => MediaStopcd,
            KEY_MEDIA_PREVIOUSSONG => MediaPrevioussong,
            KEY_MEDIA_NEXTSONG => MediaNextsong,
            KEY_MEDIA_EJECTCD => MediaEjectcd,
            KEY_MEDIA_VOLUMEUP => MediaVolumeup,
            KEY_MEDIA_VOLUMEDOWN => MediaVolumedown,
            KEY_MEDIA_MUTE => MediaMute,
            KEY_MEDIA_WWW => MediaWww,
            KEY_MEDIA_BACK => MediaBack,
            KEY_MEDIA_FORWARD => MediaForward,
            KEY_MEDIA_STOP => MediaStop,
            KEY_MEDIA_FIND => MediaFind,
            KEY_MEDIA_SCROLLUP => MediaScrollup,
            KEY_MEDIA_SCROLLDOWN => MediaScrolldown,
            KEY_MEDIA_EDIT => MediaEdit,
            KEY_MEDIA_SLEEP => MediaSleep,
            KEY_MEDIA_COFFEE => MediaCoffee,
            KEY_MEDIA_REFRESH => MediaRefresh,
            KEY_MEDIA_CALC => MediaCalc,
            _ => return Err(UnrecognizedKey(value)),
        })
    }
}

pub mod raw {
    /// No key pressed
    pub const KEY_NONE: u8 = 0x00;
    ///  Keyboard Error Roll Over - used for all slots if too many keys are pressed ("Phantom key")
    pub const KEY_ERR_OVF: u8 = 0x01;
    /// Keyboard POST Fail
    pub const KEY_ERR_POST_FAIL: u8 = 0x02;
    /// Keyboard Error Undefined
    pub const KEY_ERR_UNDEFINED: u8 = 0x03;

    /// Keyboard a and A
    pub const KEY_A: u8 = 0x04;
    /// Keyboard b and B
    pub const KEY_B: u8 = 0x05;
    /// Keyboard c and C
    pub const KEY_C: u8 = 0x06;
    /// Keyboard d and D
    pub const KEY_D: u8 = 0x07;
    /// Keyboard e and E
    pub const KEY_E: u8 = 0x08;
    /// Keyboard f and F
    pub const KEY_F: u8 = 0x09;
    /// Keyboard g and G
    pub const KEY_G: u8 = 0x0a;
    /// Keyboard h and H
    pub const KEY_H: u8 = 0x0b;
    /// Keyboard i and I
    pub const KEY_I: u8 = 0x0c;
    /// Keyboard j and J
    pub const KEY_J: u8 = 0x0d;
    /// Keyboard k and K
    pub const KEY_K: u8 = 0x0e;
    /// Keyboard l and L
    pub const KEY_L: u8 = 0x0f;
    /// Keyboard m and M
    pub const KEY_M: u8 = 0x10;
    /// Keyboard n and N
    pub const KEY_N: u8 = 0x11;
    /// Keyboard o and O
    pub const KEY_O: u8 = 0x12;
    /// Keyboard p and P
    pub const KEY_P: u8 = 0x13;
    /// Keyboard q and Q
    pub const KEY_Q: u8 = 0x14;
    /// Keyboard r and R
    pub const KEY_R: u8 = 0x15;
    /// Keyboard s and S
    pub const KEY_S: u8 = 0x16;
    /// Keyboard t and T
    pub const KEY_T: u8 = 0x17;
    /// Keyboard u and U
    pub const KEY_U: u8 = 0x18;
    /// Keyboard v and V
    pub const KEY_V: u8 = 0x19;
    /// Keyboard w and W
    pub const KEY_W: u8 = 0x1a;
    /// Keyboard x and X
    pub const KEY_X: u8 = 0x1b;
    /// Keyboard y and Y
    pub const KEY_Y: u8 = 0x1c;
    /// Keyboard z and Z
    pub const KEY_Z: u8 = 0x1d;

    /// Keyboard 1 and !
    pub const KEY_1: u8 = 0x1e;
    /// Keyboard 2 and @
    pub const KEY_2: u8 = 0x1f;
    /// Keyboard 3 and #
    pub const KEY_3: u8 = 0x20;
    /// Keyboard 4 and $
    pub const KEY_4: u8 = 0x21;
    /// Keyboard 5 and %
    pub const KEY_5: u8 = 0x22;
    /// Keyboard 6 and ^
    pub const KEY_6: u8 = 0x23;
    /// Keyboard 7 and &
    pub const KEY_7: u8 = 0x24;
    /// Keyboard 8 and *
    pub const KEY_8: u8 = 0x25;
    /// Keyboard 9 and (
    pub const KEY_9: u8 = 0x26;
    /// Keyboard 0 and )
    pub const KEY_0: u8 = 0x27;

    /// Keyboard Return (ENTER)
    pub const KEY_ENTER: u8 = 0x28;
    /// Keyboard ESCAPE
    pub const KEY_ESC: u8 = 0x29;
    /// Keyboard DELETE (Backspace)
    pub const KEY_BACKSPACE: u8 = 0x2a;
    /// Keyboard Tab
    pub const KEY_TAB: u8 = 0x2b;
    /// Keyboard Spacebar
    pub const KEY_SPACE: u8 = 0x2c;
    /// Keyboard - and _
    pub const KEY_MINUS: u8 = 0x2d;
    /// Keyboard = and +
    pub const KEY_EQUAL: u8 = 0x2e;
    /// Keyboard [ and {
    pub const KEY_LEFTBRACE: u8 = 0x2f;
    /// Keyboard ] and }
    pub const KEY_RIGHTBRACE: u8 = 0x30;
    /// Keyboard \ and |
    pub const KEY_BACKSLASH: u8 = 0x31;
    /// Keyboard Non-US # and ~
    pub const KEY_HASHTILDE_NON_US: u8 = 0x32;
    /// Keyboard ; and :
    pub const KEY_SEMICOLON: u8 = 0x33;
    /// Keyboard ' and "
    pub const KEY_APOSTROPHE: u8 = 0x34;
    /// Keyboard ` and ~
    pub const KEY_GRAVE: u8 = 0x35;
    /// Keyboard , and <
    pub const KEY_COMMA: u8 = 0x36;
    /// Keyboard . and >
    pub const KEY_DOT: u8 = 0x37;
    /// Keyboard / and ?
    pub const KEY_SLASH: u8 = 0x38;
    /// Keyboard Caps Lock
    pub const KEY_CAPSLOCK: u8 = 0x39;

    /// Keyboard F1
    pub const KEY_F1: u8 = 0x3a;
    /// Keyboard F2
    pub const KEY_F2: u8 = 0x3b;
    /// Keyboard F3
    pub const KEY_F3: u8 = 0x3c;
    /// Keyboard F4
    pub const KEY_F4: u8 = 0x3d;
    /// Keyboard F5
    pub const KEY_F5: u8 = 0x3e;
    /// Keyboard F6
    pub const KEY_F6: u8 = 0x3f;
    /// Keyboard F7
    pub const KEY_F7: u8 = 0x40;
    /// Keyboard F8
    pub const KEY_F8: u8 = 0x41;
    /// Keyboard F9
    pub const KEY_F9: u8 = 0x42;
    /// Keyboard F10
    pub const KEY_F10: u8 = 0x43;
    /// Keyboard F11
    pub const KEY_F11: u8 = 0x44;
    /// Keyboard F12
    pub const KEY_F12: u8 = 0x45;

    /// Keyboard Print Screen
    pub const KEY_PRINTSCREEN: u8 = 0x46;
    /// Keyboard Scroll Lock
    pub const KEY_SCROLLLOCK: u8 = 0x47;
    /// Keyboard Pause
    pub const KEY_PAUSE: u8 = 0x48;
    /// Keyboard Insert
    pub const KEY_INSERT: u8 = 0x49;
    /// Keyboard Home
    pub const KEY_HOME: u8 = 0x4a;
    /// Keyboard Page Up
    pub const KEY_PAGEUP: u8 = 0x4b;
    /// Keyboard Delete Forward
    pub const KEY_DELETE: u8 = 0x4c;
    /// Keyboard End
    pub const KEY_END: u8 = 0x4d;
    /// Keyboard Page Down
    pub const KEY_PAGEDOWN: u8 = 0x4e;
    /// Keyboard Right Arrow
    pub const KEY_RIGHT: u8 = 0x4f;
    /// Keyboard Left Arrow
    pub const KEY_LEFT: u8 = 0x50;
    /// Keyboard Down Arrow
    pub const KEY_DOWN: u8 = 0x51;
    /// Keyboard Up Arrow
    pub const KEY_UP: u8 = 0x52;

    /// Keyboard Num Lock and Clear
    pub const KEY_NUMLOCK: u8 = 0x53;
    /// Keypad /
    pub const KEY_KPSLASH: u8 = 0x54;
    /// Keypad *
    pub const KEY_KPASTERISK: u8 = 0x55;
    /// Keypad -
    pub const KEY_KPMINUS: u8 = 0x56;
    /// Keypad +
    pub const KEY_KPPLUS: u8 = 0x57;
    /// Keypad ENTER
    pub const KEY_KPENTER: u8 = 0x58;
    /// Keypad 1 and End
    pub const KEY_KP1: u8 = 0x59;
    /// Keypad 2 and Down Arrow
    pub const KEY_KP2: u8 = 0x5a;
    /// Keypad 3 and PageDn
    pub const KEY_KP3: u8 = 0x5b;
    /// Keypad 4 and Left Arrow
    pub const KEY_KP4: u8 = 0x5c;
    /// Keypad 5
    pub const KEY_KP5: u8 = 0x5d;
    /// Keypad 6 and Right Arrow
    pub const KEY_KP6: u8 = 0x5e;
    /// Keypad 7 and Home
    pub const KEY_KP7: u8 = 0x5f;
    /// Keypad 8 and Up Arrow
    pub const KEY_KP8: u8 = 0x60;
    /// Keypad 9 and Page Up
    pub const KEY_KP9: u8 = 0x61;
    /// Keypad 0 and Insert
    pub const KEY_KP0: u8 = 0x62;
    /// Keypad . and Delete
    pub const KEY_KPDOT: u8 = 0x63;

    /// Keyboard Non-US \ and |
    pub const KEY_BACKSLASH_NON_US: u8 = 0x64;
    /// Keyboard Application
    pub const KEY_COMPOSE: u8 = 0x65;
    /// Keyboard Power
    pub const KEY_POWER: u8 = 0x66;
    /// Keypad =
    pub const KEY_KPEQUAL: u8 = 0x67;

    /// Keyboard F13
    pub const KEY_F13: u8 = 0x68;
    /// Keyboard F14
    pub const KEY_F14: u8 = 0x69;
    /// Keyboard F15
    pub const KEY_F15: u8 = 0x6a;
    /// Keyboard F16
    pub const KEY_F16: u8 = 0x6b;
    /// Keyboard F17
    pub const KEY_F17: u8 = 0x6c;
    /// Keyboard F18
    pub const KEY_F18: u8 = 0x6d;
    /// Keyboard F19
    pub const KEY_F19: u8 = 0x6e;
    /// Keyboard F20
    pub const KEY_F20: u8 = 0x6f;
    /// Keyboard F21
    pub const KEY_F21: u8 = 0x70;
    /// Keyboard F22
    pub const KEY_F22: u8 = 0x71;
    /// Keyboard F23
    pub const KEY_F23: u8 = 0x72;
    /// Keyboard F24
    pub const KEY_F24: u8 = 0x73;

    /// Keyboard Execute
    pub const KEY_OPEN: u8 = 0x74;
    /// Keyboard Help
    pub const KEY_HELP: u8 = 0x75;
    /// Keyboard Menu
    pub const KEY_PROPS: u8 = 0x76;
    /// Keyboard Select
    pub const KEY_FRONT: u8 = 0x77;
    /// Keyboard Stop
    pub const KEY_STOP: u8 = 0x78;
    /// Keyboard Again
    pub const KEY_AGAIN: u8 = 0x79;
    /// Keyboard Undo
    pub const KEY_UNDO: u8 = 0x7a;
    /// Keyboard Cut
    pub const KEY_CUT: u8 = 0x7b;
    /// Keyboard Copy
    pub const KEY_COPY: u8 = 0x7c;
    /// Keyboard Paste
    pub const KEY_PASTE: u8 = 0x7d;
    /// Keyboard Find
    pub const KEY_FIND: u8 = 0x7e;
    /// Keyboard Mute
    pub const KEY_MUTE: u8 = 0x7f;
    /// Keyboard Volume Up
    pub const KEY_VOLUMEUP: u8 = 0x80;
    /// Keyboard Volume Down
    pub const KEY_VOLUMEDOWN: u8 = 0x81;
    // 0x82  Keyboard Locking Caps Lock
    // 0x83  Keyboard Locking Num Lock
    // 0x84  Keyboard Locking Scroll Lock
    /// Keypad Comma
    pub const KEY_KPCOMMA: u8 = 0x85;
    // 0x86  Keypad Equal Sign
    /// Keyboard International1, allocated to the ろ (ro) key
    pub const KEY_INTERNATIONAL1: u8 = 0x87;
    /// Keyboard International1, allocated to the ろ (ro) key
    pub const KEY_RO: u8 = KEY_INTERNATIONAL1;
    /// Keyboard International2, allocated to the カタカナ/ひらがな (hiragana/katakana toggle) key
    pub const KEY_INTERNATIONAL2: u8 = 0x88;
    /// Keyboard International2, allocated to the カタカナ/ひらがな (hiragana/katakana toggle) key
    pub const KEY_KATAKANAHIRAGANA: u8 = KEY_INTERNATIONAL2;
    /// Keyboard International3, allocated to the ¥ (yen) key
    pub const KEY_INTERNATIONAL3: u8 = 0x89;
    /// Keyboard International3, allocated to the ¥ (yen) key
    pub const KEY_YEN: u8 = KEY_INTERNATIONAL3;
    /// Keyboard International4, allocated to the 変換 (henkan, convert from kana to kanji) key
    pub const KEY_INTERNATIONAL4: u8 = 0x8a;
    /// Keyboard International4, allocated to the 変換 (henkan, convert from kana to kanji) key
    pub const KEY_HENKAN: u8 = KEY_INTERNATIONAL4;
    /// Keyboard International5, allocated to the 無変換 (muhenkan, don't convert from kana to kanji) key
    pub const KEY_INTERNATIONAL5: u8 = 0x8b;
    /// Keyboard International5, allocated to the 無変換 (muhenkan, don't convert from kana to kanji) key
    pub const KEY_MUHENKAN: u8 = KEY_INTERNATIONAL5;
    /// Keyboard International6, allocated to the Japanese keypad comma key found on PC98 keyboards (?)
    pub const KEY_INTERNATIONAL6: u8 = 0x8c;
    /// Keyboard International6, allocated to the Japanese keypad comma key found on PC98 keyboards (?)
    pub const KEY_KPJPCOMMA: u8 = KEY_INTERNATIONAL6;
    // 0x8d  Keyboard International7
    // 0x8e  Keyboard International8
    // 0x8f  Keyboard International9
    /// Keyboard LANG1, allocated to the 한/영 (han/yeong, hangul/english toggle) key
    pub const KEY_LANG1: u8 = 0x90;
    /// Keyboard LANG1, allocated to the 한/영 (han/yeong, hangul/english toggle) key
    pub const KEY_HANGUL: u8 = KEY_LANG1;
    /// Keyboard LANG2, allocated to the 한자 (hanja, convert from hangul to hanja) key
    pub const KEY_LANG2: u8 = 0x91;
    /// Keyboard LANG2, allocated to the 한자 (hanja, convert from hangul to hanja) key
    pub const KEY_HANJA: u8 = KEY_LANG2;
    /// Keyboard LANG3, allocated to the dedicated katakana key on Japanese word-processing keyboards
    /// (citation needed? i can't find anything that doesn't word-for-word copy the same phrasing)
    pub const KEY_LANG3: u8 = 0x92;
    /// Keyboard LANG3, allocated to the dedicated katakana key on Japanese word-processing keyboards
    /// (citation needed? i can't find anything that doesn't word-for-word copy the same phrasing)
    pub const KEY_KATAKANA: u8 = KEY_LANG3;
    /// Keyboard LANG4, allocated to the dedicated hiragana key on Japanese word-processing keyboards
    /// (citation needed? i can't find anything that doesn't word-for-word copy the same phrasing)
    pub const KEY_LANG4: u8 = 0x93;
    /// Keyboard LANG4, allocated to the dedicated hiragana key on Japanese word-processing keyboards
    /// (citation needed? i can't find anything that doesn't word-for-word copy the same phrasing)
    pub const KEY_HIRAGANA: u8 = KEY_LANG4;
    /// Keyboard LANG5, allocated to the 半角/全角 (hankaku/zenkaku, halfwidth/fullwidth toggle) key
    pub const KEY_LANG5: u8 = 0x94;
    /// Keyboard LANG5, allocated to the 半角/全角 (hankaku/zenkaku, halfwidth/fullwidth toggle) key
    pub const KEY_HANKAKUZENKAKU: u8 = KEY_LANG5;
    // 0x95  Keyboard LANG6
    // 0x96  Keyboard LANG7
    // 0x97  Keyboard LANG8
    // 0x98  Keyboard LANG9
    // 0x99  Keyboard Alternate Erase
    // 0x9a  Keyboard SysReq/Attention
    // 0x9b  Keyboard Cancel
    // 0x9c  Keyboard Clear
    // 0x9d  Keyboard Prior
    // 0x9e  Keyboard Return
    // 0x9f  Keyboard Separator
    // 0xa0  Keyboard Out
    // 0xa1  Keyboard Oper
    // 0xa2  Keyboard Clear/Again
    // 0xa3  Keyboard CrSel/Props
    // 0xa4  Keyboard ExSel

    // 0xb0  Keypad 00
    // 0xb1  Keypad 000
    // 0xb2  Thousands Separator
    // 0xb3  Decimal Separator
    // 0xb4  Currency Unit
    // 0xb5  Currency Sub-unit
    /// Keypad (
    pub const KEY_KPLEFTPAREN: u8 = 0xb6;
    /// Keypad )
    pub const KEY_KPRIGHTPAREN: u8 = 0xb7;
    // 0xb8  Keypad {
    // 0xb9  Keypad }
    // 0xba  Keypad Tab
    // 0xbb  Keypad Backspace
    // 0xbc  Keypad A
    // 0xbd  Keypad B
    // 0xbe  Keypad C
    // 0xbf  Keypad D
    // 0xc0  Keypad E
    // 0xc1  Keypad F
    // 0xc2  Keypad XOR
    // 0xc3  Keypad ^
    // 0xc4  Keypad %
    // 0xc5  Keypad <
    // 0xc6  Keypad >
    // 0xc7  Keypad &
    // 0xc8  Keypad &&
    // 0xc9  Keypad |
    // 0xca  Keypad ||
    // 0xcb  Keypad :
    // 0xcc  Keypad #
    // 0xcd  Keypad Space
    // 0xce  Keypad @
    // 0xcf  Keypad !
    // 0xd0  Keypad Memory Store
    // 0xd1  Keypad Memory Recall
    // 0xd2  Keypad Memory Clear
    // 0xd3  Keypad Memory Add
    // 0xd4  Keypad Memory Subtract
    // 0xd5  Keypad Memory Multiply
    // 0xd6  Keypad Memory Divide
    // 0xd7  Keypad +/-
    // 0xd8  Keypad Clear
    // 0xd9  Keypad Clear Entry
    // 0xda  Keypad Binary
    // 0xdb  Keypad Octal
    // 0xdc  Keypad Decimal
    // 0xdd  Keypad Hexadecimal

    /// Keyboard Left Control
    ///
    /// You probably want to check [`Modifiers`](super::Modifiers) instead!
    pub const KEY_LEFTCTRL: u8 = 0xe0;
    /// Keyboard Left Shift
    ///
    /// You probably want to check [`Modifiers`](super::Modifiers) instead!
    pub const KEY_LEFTSHIFT: u8 = 0xe1;
    /// Keyboard Left Alt
    ///
    /// You probably want to check [`Modifiers`](super::Modifiers) instead!
    pub const KEY_LEFTALT: u8 = 0xe2;
    /// Keyboard Left Super
    ///
    /// You probably want to check [`Modifiers`](super::Modifiers) instead!
    pub const KEY_LEFTSUPER: u8 = 0xe3;
    /// Keyboard Right Control
    ///
    /// You probably want to check [`Modifiers`](super::Modifiers) instead!
    pub const KEY_RIGHTCTRL: u8 = 0xe4;
    /// Keyboard Right Shift
    ///
    /// You probably want to check [`Modifiers`](super::Modifiers) instead!
    pub const KEY_RIGHTSHIFT: u8 = 0xe5;
    /// Keyboard Right Alt
    ///
    /// You probably want to check [`Modifiers`](super::Modifiers) instead!
    pub const KEY_RIGHTALT: u8 = 0xe6;
    /// Keyboard Right Super
    ///
    /// You probably want to check [`Modifiers`](super::Modifiers) instead!
    pub const KEY_RIGHTSUPER: u8 = 0xe7;

    pub const KEY_MEDIA_PLAYPAUSE: u8 = 0xe8;
    pub const KEY_MEDIA_STOPCD: u8 = 0xe9;
    pub const KEY_MEDIA_PREVIOUSSONG: u8 = 0xea;
    pub const KEY_MEDIA_NEXTSONG: u8 = 0xeb;
    pub const KEY_MEDIA_EJECTCD: u8 = 0xec;
    pub const KEY_MEDIA_VOLUMEUP: u8 = 0xed;
    pub const KEY_MEDIA_VOLUMEDOWN: u8 = 0xee;
    pub const KEY_MEDIA_MUTE: u8 = 0xef;
    pub const KEY_MEDIA_WWW: u8 = 0xf0;
    pub const KEY_MEDIA_BACK: u8 = 0xf1;
    pub const KEY_MEDIA_FORWARD: u8 = 0xf2;
    pub const KEY_MEDIA_STOP: u8 = 0xf3;
    pub const KEY_MEDIA_FIND: u8 = 0xf4;
    pub const KEY_MEDIA_SCROLLUP: u8 = 0xf5;
    pub const KEY_MEDIA_SCROLLDOWN: u8 = 0xf6;
    pub const KEY_MEDIA_EDIT: u8 = 0xf7;
    pub const KEY_MEDIA_SLEEP: u8 = 0xf8;
    pub const KEY_MEDIA_COFFEE: u8 = 0xf9;
    pub const KEY_MEDIA_REFRESH: u8 = 0xfa;
    pub const KEY_MEDIA_CALC: u8 = 0xfb;
}
