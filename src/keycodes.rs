/// Keycode of mouse buttons
/// 
/// # Example
/// 
/// ```
/// if manager.get_mouse_button(Button::LBUTTON) == Action::Release {
///     println!("Left mouse button is released");
/// }
/// ```
#[allow(non_snake_case, non_upper_case_globals)]
pub mod Button {
    /// Keycode of left mouse button
    pub const LBUTTON: usize = 0x01;
    /// Keycode of right mouse button
    pub const RBUTTON: usize = 0x02;
    /// Keycode of middle mouse button
    pub const MBUTTON: usize = 0x04;
    /// Keycode of x mouse button 1
    pub const XBUTTON1: usize = 0x05;
    /// Keycode of x mouse button 2
    pub const XBUTTON2: usize = 0x06;
}

/// Keycodes of the keys in the keyboard
/// 
/// # Example
/// 
/// ```
/// if manager.get_key(Y) == Action::Down {
///     println!("Y key is down");
/// }
/// ```
#[allow(non_snake_case, non_upper_case_globals)]
pub mod Key {
    pub const CANCEL: usize = 0x03;
    pub const BACKSPACE: usize = 0x08;
    pub const TAB: usize = 0x09;
    pub const CLEAR: usize = 0x0C;
    pub const RETURN: usize = 0x0D;
    pub const SHIFT: usize = 0x10;
    pub const CONTROL: usize = 0x11;
    pub const ALT: usize = 0x12;
    pub const PAUSE: usize = 0x13;
    pub const CAPITAL: usize = 0x14;
    pub const KANA: usize = 0x15;
    pub const HANGEUL: usize = 0x15;
    pub const HANGUL: usize = 0x15;
    pub const JUNJA: usize = 0x17;
    pub const FINAL: usize = 0x18;
    pub const HANJA: usize = 0x19;
    pub const KANJI: usize = 0x19;
    pub const ESCAPE: usize = 0x1B;
    pub const CONVERT: usize = 0x1C;
    pub const NONCONVERT: usize = 0x1D;
    pub const ACCEPT: usize = 0x1E;
    pub const MODECHANGE: usize = 0x1F;
    pub const SPACE: usize = 0x20;
    pub const PRIOR: usize = 0x21;
    pub const NEXT: usize = 0x22;
    pub const END: usize = 0x23;
    pub const HOME: usize = 0x24;
    pub const LEFT: usize = 0x25;
    pub const UP: usize = 0x26;
    pub const RIGHT: usize = 0x27;
    pub const DOWN: usize = 0x28;
    pub const SELECT: usize = 0x29;
    pub const PRINT: usize = 0x2A;
    pub const EXECUTE: usize = 0x2B;
    pub const SNAPSHOT: usize = 0x2C;
    pub const INSERT: usize = 0x2D;
    pub const DELETE: usize = 0x2E;
    pub const HELP: usize = 0x2F;
    pub const LWIN: usize = 0x5B;
    pub const RWIN: usize = 0x5C;
    pub const APPS: usize = 0x5D;
    pub const SLEEP: usize = 0x5F;
    pub const NUMPAD0: usize = 0x60;
    pub const NUMPAD1: usize = 0x61;
    pub const NUMPAD2: usize = 0x62;
    pub const NUMPAD3: usize = 0x63;
    pub const NUMPAD4: usize = 0x64;
    pub const NUMPAD5: usize = 0x65;
    pub const NUMPAD6: usize = 0x66;
    pub const NUMPAD7: usize = 0x67;
    pub const NUMPAD8: usize = 0x68;
    pub const NUMPAD9: usize = 0x69;
    pub const MULTIPLY: usize = 0x6A;
    pub const ADD: usize = 0x6B;
    pub const SEPARATOR: usize = 0x6C;
    pub const SUBTRACT: usize = 0x6D;
    pub const DECIMAL: usize = 0x6E;
    pub const DIVIDE: usize = 0x6F;
    pub const F1: usize = 0x70;
    pub const F2: usize = 0x71;
    pub const F3: usize = 0x72;
    pub const F4: usize = 0x73;
    pub const F5: usize = 0x74;
    pub const F6: usize = 0x75;
    pub const F7: usize = 0x76;
    pub const F8: usize = 0x77;
    pub const F9: usize = 0x78;
    pub const F10: usize = 0x79;
    pub const F11: usize = 0x7A;
    pub const F12: usize = 0x7B;
    pub const F13: usize = 0x7C;
    pub const F14: usize = 0x7D;
    pub const F15: usize = 0x7E;
    pub const F16: usize = 0x7F;
    pub const F17: usize = 0x80;
    pub const F18: usize = 0x81;
    pub const F19: usize = 0x82;
    pub const F20: usize = 0x83;
    pub const F21: usize = 0x84;
    pub const F22: usize = 0x85;
    pub const F23: usize = 0x86;
    pub const F24: usize = 0x87;
    pub const NAVIGATION_VIEW: usize = 0x88;
    pub const NAVIGATION_MENU: usize = 0x89;
    pub const NAVIGATION_UP: usize = 0x8A;
    pub const NAVIGATION_DOWN: usize = 0x8B;
    pub const NAVIGATION_LEFT: usize = 0x8C;
    pub const NAVIGATION_RIGHT: usize = 0x8D;
    pub const NAVIGATION_ACCEPT: usize = 0x8E;
    pub const NAVIGATION_CANCEL: usize = 0x8F;
    pub const NUMLOCK: usize = 0x90;
    pub const SCROLL: usize = 0x91;
    pub const OEM_NEC_EQUAL: usize = 0x92;
    pub const OEM_FJ_JISHO: usize = 0x92;
    pub const OEM_FJ_MASSHOU: usize = 0x93;
    pub const OEM_FJ_TOUROKU: usize = 0x94;
    pub const OEM_FJ_LOYA: usize = 0x95;
    pub const OEM_FJ_ROYA: usize = 0x96;
    pub const LSHIFT: usize = 0xA0;
    pub const RSHIFT: usize = 0xA1;
    pub const LCONTROL: usize = 0xA2;
    pub const RCONTROL: usize = 0xA3;
    pub const LMENU: usize = 0xA4;
    pub const RMENU: usize = 0xA5;
    pub const BROWSER_BACK: usize = 0xA6;
    pub const BROWSER_FORWARD: usize = 0xA7;
    pub const BROWSER_REFRESH: usize = 0xA8;
    pub const BROWSER_STOP: usize = 0xA9;
    pub const BROWSER_SEARCH: usize = 0xAA;
    pub const BROWSER_FAVORITES: usize = 0xAB;
    pub const BROWSER_HOME: usize = 0xAC;
    pub const VOLUME_MUTE: usize = 0xAD;
    pub const VOLUME_DOWN: usize = 0xAE;
    pub const VOLUME_UP: usize = 0xAF;
    pub const MEDIA_NEXT_TRACK: usize = 0xB0;
    pub const MEDIA_PREV_TRACK: usize = 0xB1;
    pub const MEDIA_STOP: usize = 0xB2;
    pub const MEDIA_PLAY_PAUSE: usize = 0xB3;
    pub const LAUNCH_MAIL: usize = 0xB4;
    pub const LAUNCH_MEDIA_SELECT: usize = 0xB5;
    pub const LAUNCH_APP1: usize = 0xB6;
    pub const LAUNCH_APP2: usize = 0xB7;
    pub const OEM_1: usize = 0xBA;
    pub const OEM_PLUS: usize = 0xBB;
    pub const OEM_COMMA: usize = 0xBC;
    pub const OEM_MINUS: usize = 0xBD;
    pub const OEM_PERIOD: usize = 0xBE;
    pub const OEM_2: usize = 0xBF;
    pub const OEM_3: usize = 0xC0;
    pub const GAMEPAD_A: usize = 0xC3;
    pub const GAMEPAD_B: usize = 0xC4;
    pub const GAMEPAD_X: usize = 0xC5;
    pub const GAMEPAD_Y: usize = 0xC6;
    pub const GAMEPAD_RIGHT_SHOULDER: usize = 0xC7;
    pub const GAMEPAD_LEFT_SHOULDER: usize = 0xC8;
    pub const GAMEPAD_LEFT_TRIGGER: usize = 0xC9;
    pub const GAMEPAD_RIGHT_TRIGGER: usize = 0xCA;
    pub const GAMEPAD_DPAD_UP: usize = 0xCB;
    pub const GAMEPAD_DPAD_DOWN: usize = 0xCC;
    pub const GAMEPAD_DPAD_LEFT: usize = 0xCD;
    pub const GAMEPAD_DPAD_RIGHT: usize = 0xCE;
    pub const GAMEPAD_MENU: usize = 0xCF;
    pub const GAMEPAD_VIEW: usize = 0xD0;
    pub const GAMEPAD_LEFT_THUMBSTICK_BUTTON: usize = 0xD1;
    pub const GAMEPAD_RIGHT_THUMBSTICK_BUTTON: usize = 0xD2;
    pub const GAMEPAD_LEFT_THUMBSTICK_UP: usize = 0xD3;
    pub const GAMEPAD_LEFT_THUMBSTICK_DOWN: usize = 0xD4;
    pub const GAMEPAD_LEFT_THUMBSTICK_RIGHT: usize = 0xD5;
    pub const GAMEPAD_LEFT_THUMBSTICK_LEFT: usize = 0xD6;
    pub const GAMEPAD_RIGHT_THUMBSTICK_UP: usize = 0xD7;
    pub const GAMEPAD_RIGHT_THUMBSTICK_DOWN: usize = 0xD8;
    pub const GAMEPAD_RIGHT_THUMBSTICK_RIGHT: usize = 0xD9;
    pub const GAMEPAD_RIGHT_THUMBSTICK_LEFT: usize = 0xDA;
    pub const OEM_4: usize = 0xDB;
    pub const OEM_5: usize = 0xDC;
    pub const OEM_6: usize = 0xDD;
    pub const OEM_7: usize = 0xDE;
    pub const OEM_8: usize = 0xDF;
    pub const OEM_AX: usize = 0xE1;
    pub const OEM_102: usize = 0xE2;
    pub const ICO_HELP: usize = 0xE3;
    pub const ICO_00: usize = 0xE4;
    pub const PROCESSKEY: usize = 0xE5;
    pub const ICO_CLEAR: usize = 0xE6;
    pub const PACKET: usize = 0xE7;
    pub const OEM_RESET: usize = 0xE9;
    pub const OEM_JUMP: usize = 0xEA;
    pub const OEM_PA1: usize = 0xEB;
    pub const OEM_PA2: usize = 0xEC;
    pub const OEM_PA3: usize = 0xED;
    pub const OEM_WSCTRL: usize = 0xEE;
    pub const OEM_CUSEL: usize = 0xEF;
    pub const OEM_ATTN: usize = 0xF0;
    pub const OEM_FINISH: usize = 0xF1;
    pub const OEM_COPY: usize = 0xF2;
    pub const OEM_AUTO: usize = 0xF3;
    pub const OEM_ENLW: usize = 0xF4;
    pub const OEM_BACKTAB: usize = 0xF5;
    pub const ATTN: usize = 0xF6;
    pub const CRSEL: usize = 0xF7;
    pub const EXSEL: usize = 0xF8;
    pub const EREOF: usize = 0xF9;
    pub const PLAY: usize = 0xFA;
    pub const ZOOM: usize = 0xFB;
    pub const NONAME: usize = 0xFC;
    pub const PA1: usize = 0xFD;
    pub const OEM_CLEAR: usize = 0xFE;
    pub const A: usize = 'A' as u8 as usize;
    pub const B: usize = 'B' as u8 as usize;
    pub const C: usize = 'C' as u8 as usize;
    pub const D: usize = 'D' as u8 as usize;
    pub const E: usize = 'E' as u8 as usize;
    pub const F: usize = 'F' as u8 as usize;
    pub const G: usize = 'G' as u8 as usize;
    pub const H: usize = 'H' as u8 as usize;
    pub const I: usize = 'I' as u8 as usize;
    pub const J: usize = 'J' as u8 as usize;
    pub const K: usize = 'K' as u8 as usize;
    pub const L: usize = 'L' as u8 as usize;
    pub const M: usize = 'M' as u8 as usize;
    pub const N: usize = 'N' as u8 as usize;
    pub const O: usize = 'O' as u8 as usize;
    pub const P: usize = 'P' as u8 as usize;
    pub const Q: usize = 'Q' as u8 as usize;
    pub const R: usize = 'R' as u8 as usize;
    pub const S: usize = 'S' as u8 as usize;
    pub const T: usize = 'T' as u8 as usize;
    pub const U: usize = 'U' as u8 as usize;
    pub const V: usize = 'V' as u8 as usize;
    pub const W: usize = 'W' as u8 as usize;
    pub const X: usize = 'X' as u8 as usize;
    pub const Y: usize = 'Y' as u8 as usize;
    pub const Z: usize = 'Z' as u8 as usize;
    pub const a: usize = 'a' as u8 as usize;
    pub const b: usize = 'b' as u8 as usize;
    pub const c: usize = 'c' as u8 as usize;
    pub const d: usize = 'd' as u8 as usize;
    pub const e: usize = 'e' as u8 as usize;
    pub const f: usize = 'f' as u8 as usize;
    pub const g: usize = 'g' as u8 as usize;
    pub const h: usize = 'h' as u8 as usize;
    pub const i: usize = 'i' as u8 as usize;
    pub const j: usize = 'j' as u8 as usize;
    pub const k: usize = 'k' as u8 as usize;
    pub const l: usize = 'l' as u8 as usize;
    pub const m: usize = 'm' as u8 as usize;
    pub const n: usize = 'n' as u8 as usize;
    pub const o: usize = 'o' as u8 as usize;
    pub const p: usize = 'p' as u8 as usize;
    pub const q: usize = 'q' as u8 as usize;
    pub const r: usize = 'r' as u8 as usize;
    pub const s: usize = 's' as u8 as usize;
    pub const t: usize = 't' as u8 as usize;
    pub const u: usize = 'u' as u8 as usize;
    pub const v: usize = 'v' as u8 as usize;
    pub const w: usize = 'w' as u8 as usize;
    pub const x: usize = 'x' as u8 as usize;
    pub const y: usize = 'y' as u8 as usize;
    pub const z: usize = 'z' as u8 as usize;
    pub const NUM0: usize = '0' as u8 as usize;
    pub const NUM1: usize = '1' as u8 as usize;
    pub const NUM2: usize = '2' as u8 as usize;
    pub const NUM3: usize = '3' as u8 as usize;
    pub const NUM4: usize = '4' as u8 as usize;
    pub const NUM5: usize = '5' as u8 as usize;
    pub const NUM6: usize = '6' as u8 as usize;
    pub const NUM7: usize = '7' as u8 as usize;
    pub const NUM8: usize = '8' as u8 as usize;
    pub const NUM9: usize = '9' as u8 as usize;
}