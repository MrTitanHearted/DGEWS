use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Wstring {
    pub(crate) data: Vec<u16>,
}

impl Wstring {
    pub fn new() -> Self {
        return Self::default();
    }

    pub fn len(&self) -> usize {
        return self.data.len() - 1usize;
    }

    pub fn from(data: &str) -> Self {
        let mut wstr: Vec<u16> = data.encode_utf16().collect();
        wstr.push(0);

        return Self { data: wstr };
    }

    pub fn as_ptr(&self) -> *const u16 {
        return self.data.as_ptr();
    }

    pub fn as_mut_ptr(&mut self) -> *mut u16 {
        return self.data.as_mut_ptr();
    }

    pub fn set(&mut self, data: &str) {
        self.data = data.encode_utf16().collect();
        self.data.push(0);
    }

    pub fn push(&mut self, data: &str) {
        let mut wstring: String = String::from_utf16_lossy(&self.data);
        wstring.push_str(data);
        self.data = wstring.encode_utf16().collect();
        self.data.push(0);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Astring {
    pub(crate) data: Vec<i8>,
}

impl Astring {
    pub fn new() -> Self {
        return Self::default();
    }

    pub fn len(&self) -> usize {
        return self.data.len() - 1usize;
    }

    pub fn from(data: &str) -> Self {
        let ustr: Vec<u8> = data.bytes().collect();
        let mut astr: Vec<i8> = Vec::new();
        for char in ustr {
            astr.push(char as i8);
        }
        astr.push(0);

        return Self { data: astr };
    }

    pub fn as_ptr(&self) -> *const i8 {
        return self.data.as_ptr();
    }

    pub fn as_mut_ptr(&mut self) -> *mut i8 {
        return self.data.as_mut_ptr();
    }

    pub fn set(&mut self, data: &str) {
        let ustr: Vec<u8> = data.bytes().collect();
        let mut astr: Vec<i8> = Vec::new();
        for char in ustr {
            astr.push(char as i8);
        }
        astr.push(0);
        self.data = astr;
    }

    pub fn push(&mut self, data: &str) {
        let mut astr: Vec<u8> = Vec::new();
        for achar in self.data.clone() {
            astr.push(achar as u8);
        }
        let mut astring: String = String::from_utf8_lossy(&astr).to_string();
        astring.push_str(data);
        let ustr: Vec<u8> = astring.bytes().collect();
        let mut astr: Vec<i8> = Vec::new();
        for char in ustr {
            astr.push(char as i8);
        }
        astr.push(0);
        self.data = astr;
    }
}

pub fn wchar(data: &str) -> *const u16 {
    return Wstring::from(data).as_ptr();
}

pub fn achar(data: &str) -> *const i8 {
    return Astring::from(data).as_ptr();
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

impl Size {
    pub fn new(width: i32, height: i32) -> Self {
        return Self { width, height };
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        return Self { x, y };
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}

pub(crate) fn load_icon(path: &str) -> *mut winapi::ctypes::c_void {
    return unsafe {
        winapi::um::winuser::LoadImageW(
            std::ptr::null_mut(),
            wchar(path),
            winapi::um::winuser::IMAGE_ICON,
            0,
            0,
            winapi::um::winuser::LR_LOADFROMFILE,
        )
    };
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum Theme {
    Dark,
    #[default]
    Light,
}

impl std::fmt::Debug for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dark => write!(f, "Theme::Dark"),
            Self::Light => write!(f, "Theme::Light"),
        }
    }
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dark => write!(f, "Theme::Dark"),
            Self::Light => write!(f, "Theme::Light"),
        }
    }
}
