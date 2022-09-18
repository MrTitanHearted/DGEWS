use std::fmt::Display;

/// # Wide String
///
/// 'Wstring' is a struct which contains strings in the form of wide characters. (Equals to LPWSTR in c++)
/// # Example
///
/// ```
/// let title = Wstring::from("Hello, World!");
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Wstring {
    pub(crate) data: Vec<u16>,
}

impl Wstring {
    /// Creates new empty Wstring
    ///
    /// # Example
    ///
    /// ```
    /// let wstring = Wstring::new();
    /// ```
    pub fn new() -> Self {
        return Self::default();
    }

    /// Returns the lenth of the wide string (excluding \0 terminator character)
    ///
    /// # Example
    ///
    /// ```
    /// let wstring = Wstring::from("Rust Lang");
    ///
    /// assert_eq!(wstring.len(), 9);
    /// ```
    pub fn len(&self) -> usize {
        return self.data.len() - 1usize;
    }

    /// Creates Wstring from &str type
    ///
    /// # Example
    ///
    /// ```
    /// let wstring = Wstring::from("Rust Lang");
    /// ```
    pub fn from(data: &str) -> Self {
        let mut wstr: Vec<u16> = data.encode_utf16().collect();
        wstr.push(0);

        return Self { data: wstr };
    }

    /// Returns a pointer of utf-16
    ///
    /// # Example
    ///
    /// ```
    /// let wstring = Wstring::from("Rust Lang");
    /// func(wstring.as_ptr())
    /// // where func uses a memory pointer to the wstring
    /// ```
    pub fn as_ptr(&self) -> *const u16 {
        return self.data.as_ptr();
    }

    /// Returns a mutable pointer of utf-16
    ///
    /// # Example
    ///
    /// ```
    /// let wstring = Wstring::from("Rust Lang");
    /// func(wstring.as_mut_ptr())
    /// // where func uses a memory pointer to the mutable wstring
    /// ```
    pub fn as_mut_ptr(&mut self) -> *mut u16 {
        return self.data.as_mut_ptr();
    }

    /// Populates the wstring with contents.
    /// The same as from() function, however set() uses already created wstring
    ///
    /// # Example
    ///
    /// ```
    /// let mut wstring = Wstring::new();
    /// wstring.set("Hello");
    /// assert_eq!(wstring.len(), 5);
    /// ```
    ///
    /// ```
    /// let mut title = Wstring::from("DGE");
    /// assert_eq!(title.len(), 3);
    ///
    /// title.set("DGEWS");
    /// assert_eq!(title.len(), 5);
    /// ```
    pub fn set(&mut self, data: &str) {
        self.data = data.encode_utf16().collect();
        self.data.push(0);
    }

    /// Tail adds a new slice of &str to the Wstring
    ///
    /// # Example
    ///
    /// ```
    /// let mut wstring = Wstring::new();
    /// assert_eq!(wstring.len(), 0);
    ///
    /// wstring.push("Hi");
    /// assert_eq!(wstring.len(), 2);
    ///
    /// wstring.push(" I am MrTitanHearted");
    /// assert_eq!(wstring.len(), 20);
    /// ```
    pub fn push(&mut self, data: &str) {
        let mut wstring: String = String::from_utf16_lossy(&self.data);
        wstring.push_str(data);
        self.data = wstring.encode_utf16().collect();
        self.data.push(0);
    }
}

/// # ASCII String (instead of u8, it consists of i8)
///
/// 'Astring' is a struct which contains strings in the form of small characters. (Equals to LPSTR in c++)
/// # Example
///
/// ```
/// let title = Astring::from("Hello, World!");
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Astring {
    pub(crate) data: Vec<i8>,
}

impl Astring {
    /// Creates new empty Astring
    ///
    /// # Example
    ///
    /// ```
    /// let astring = Astring::new();
    /// ```
    pub fn new() -> Self {
        return Self::default();
    }

    /// Returns the lenth of the small string (excluding \0 terminator character)
    ///
    /// # Example
    ///
    /// ```
    /// let astring = Astring::from("Rust Lang");
    ///
    /// assert_eq!(astring.len(), 9);
    /// ```
    pub fn len(&self) -> usize {
        return self.data.len() - 1usize;
    }

    /// Creates Astring from &str type
    ///
    /// # Example
    ///
    /// ```
    /// let astring = Astring::from("Rust Lang");
    /// ```
    pub fn from(data: &str) -> Self {
        let ustr: Vec<u8> = data.bytes().collect();
        let mut astr: Vec<i8> = Vec::new();
        for char in ustr {
            astr.push(char as i8);
        }
        astr.push(0);

        return Self { data: astr };
    }

    /// Returns a pointer of i8
    ///
    /// # Example
    ///
    /// ```
    /// let astring = Astring::from("Rust Lang");
    /// func(astring.as_ptr())
    /// // where func uses a memory pointer to the astring
    /// ```
    pub fn as_ptr(&self) -> *const i8 {
        return self.data.as_ptr();
    }

    /// Returns a mutable pointer of i8
    ///
    /// # Example
    ///
    /// ```
    /// let astring = Astring::from("Rust Lang");
    /// func(astring.as_mut_ptr())
    /// // where func uses a memory pointer to the mutable astring
    /// ```
    pub fn as_mut_ptr(&mut self) -> *mut i8 {
        return self.data.as_mut_ptr();
    }

    /// Populates the astring with contents.
    /// The same as from() function, however set() uses already created astring
    ///
    /// # Example
    ///
    /// ```
    /// let mut astring = Astring::new();
    /// astring.set("Hello");
    /// assert_eq!(astring.len(), 5);
    /// ```
    ///
    /// ```
    /// let mut title = Astring::from("DGE");
    /// assert_eq!(title.len(), 3);
    ///
    /// title.set("DGEWS");
    /// assert_eq!(title.len(), 5);
    /// ```
    pub fn set(&mut self, data: &str) {
        let ustr: Vec<u8> = data.bytes().collect();
        let mut astr: Vec<i8> = Vec::new();
        for char in ustr {
            astr.push(char as i8);
        }
        astr.push(0);
        self.data = astr;
    }

    /// Tail adds a new slice of &str to the Astring
    ///
    /// # Example
    ///
    /// ```
    /// let mut astring = Astring::new();
    /// assert_eq!(astring.len(), 0);
    ///
    /// astring.push("Hi");
    /// assert_eq!(astring.len(), 2);
    ///
    /// astring.push(" I am MrTitanHearted");
    /// assert_eq!(astring.len(), 20);
    /// ```
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

/// Creates and returns a pointer to Wstring
///
/// # Example
///
/// ```
/// let wstring = Wstring::from("WSTRING");
/// func(wstring.as_ptr());
///
/// func(wchar("WSTRING"));
///
/// // where func uses a constant pointer to Wstring
/// ```
pub fn wchar(data: &str) -> *mut u16 {
    return Wstring::from(data).as_mut_ptr();
}

/// Creates and returns a pointer to Astring
///
/// # Example
///
/// ```
/// let astring = Astring::from("ASTRING");
/// func(astring.as_ptr());
///
/// func(wchar("ASTRING"));
///
/// // where func uses a constant pointer to Astring
/// ```
pub fn achar(data: &str) -> *mut i8 {
    return Astring::from(data).as_mut_ptr();
}

/// A struct for holding width and height information for Window or WindowBuilder
///
/// # Example
///
/// ```
/// let size = Size::new(1366, 768);
/// assert_eq!(size.width, 1366);
/// assert_eq!(size.height, 768);
/// ```
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Size {
    /// Width i32 type
    pub width: i32,
    // Height i32 type
    pub height: i32,
}

impl Size {
    /// Creates a new Size
    ///
    /// # Example
    ///
    /// ```
    /// let size = Size::new(800, 800);
    /// ```
    pub fn new(width: i32, height: i32) -> Self {
        return Self { width, height };
    }
}

/// A 2D point
///
/// # Example
///
/// ```
/// let pos = Point::new(56, 78);
/// assert_eq!(pos.x, 56);
/// assert_eq!(pos.y, 78);
/// ```
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Point {
    /// X pos i32 type
    pub x: i32,
    /// Y pos i32 type
    pub y: i32,
}

impl Point {
    /// Creates a new Point
    ///
    /// # Example
    ///
    /// ```
    /// let point = Point::new(800, 800);
    /// ```
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

/// Theme of the GUI (only Light or Dark).
/// Light mode is default one
///
/// # Example
///
/// ```
/// let wb = WindowBuilder::default()
///     .with_theme(Theme::Light);
/// ```
#[derive(Clone, Copy, PartialEq, Default)]
pub enum Theme {
    /// Dark Mode
    Dark,
    /// Light Mode
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
