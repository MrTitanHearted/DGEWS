use winapi::{
    ctypes::*,
    shared::{minwindef::*, windef::*},
    um::{dwmapi::DwmSetWindowAttribute, libloaderapi::*, winuser::*},
};

use crate::prelude::*;

/// A handle that holds information of a window
///
/// # Example
///
/// ```ignore
/// let window = Window::default();
/// assert_eq!(window.get_title(), String::from("Direct Game Engine Window"));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Window {
    pub(crate) hwnd: HWND,
    pub(crate) title: String,
    pub(crate) pos: Point,
    pub(crate) size: Size,
}

impl Default for Window {
    fn default() -> Self {
        return Self {
            hwnd: std::ptr::null_mut(),
            title: String::from("Direct Game Engine Window"),
            pos: Point::default(),
            size: Size::new(800, 640),
        };
    }
}

impl Window {
    /// Creates a new handle with an id
    ///
    /// # Example
    ///
    /// ```ignore
    /// let window = Window::new();
    /// assert_eq!(window.get_title(), String::from("Direct Game Engine Window"));
    /// ```
    pub fn new() -> Self {
        return Self::default();
    }

    pub fn with_hwnd(mut self, hwnd: HWND) -> Self {
        self.hwnd = hwnd;
        return self;
    }

    /// Sets the title of that window
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut window = Window::default();
    /// assert_eq!(window.get_title(), String::from("Direct Game Engine Window"));
    ///
    /// window.set_title("Hello, World!");
    /// assert_eq!(window.get_title(), String::from("Hello, World!"));
    /// ```
    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_owned();
        unsafe {
            SetWindowTextW(self.hwnd, wchar(title));
        }
    }

    /// Sets the icon of the window. (It does not change its icon in properties, only in taskbar and titlebar)
    ///
    /// # Example
    ///
    /// ```ignore
    /// let window = Window::default();
    /// window.set_icon("path\\to\\your\\icon\\.ico");
    /// ```
    pub fn set_icon(&self, path: &str) {
        unsafe {
            SetClassLongPtrW(self.hwnd, GCLP_HICON, load_icon(path) as isize);
        }
    }

    /// Sets the position of the window
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut window = Window::default();
    /// assert_eq!(window.get_pos.x, 0); assert_eq!(window.get_pos.y, 0);
    ///
    /// window.set_position(Point::new(100, 200));
    /// assert_eq!(window.get_pos.x, 100); assert_eq!(window.get_pos.y, 200);
    /// ```
    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.pos = Point::new(x, y);
        unsafe {
            SetWindowPos(
                self.hwnd,
                std::ptr::null_mut(),
                x,
                y,
                0i32,
                0i32,
                SWP_DRAWFRAME | SWP_NOZORDER | SWP_NOSIZE | SWP_SHOWWINDOW,
            );
        }
    }

    /// Sets the size of the window
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut window = Window::default();
    /// assert_eq!(window.get_size().width, 800); assert_eq!(window.get_size().height, 640);
    ///
    /// window.set_size(Size::new(1024, 768));
    /// assert_eq!(window.get_size().width, 1024); assert_eq!(window.get_size().height, 768);
    /// ```
    pub fn set_size(&mut self, width: i32, height: i32) {
        self.size = Size::new(width, height);
        unsafe {
            SetWindowPos(
                self.hwnd,
                std::ptr::null_mut(),
                0i32,
                0i32,
                width,
                height,
                SWP_DRAWFRAME | SWP_NOZORDER | SWP_NOREPOSITION | SWP_SHOWWINDOW,
            );
        }
    }

    /// Returns the id of the window
    ///
    /// # Example
    ///
    /// ```ignore
    /// let window = Window::default();
    /// assert_eq!(window.get_id(), 0); // The id of the window is 0 by default.
    /// ```
    pub fn get_id(&self) -> usize {
        return self.hwnd as usize;
    }

    /// Returns the title of the window
    ///
    /// # Example
    ///
    /// ```ignore
    /// let window = Window::default();
    /// assert_eq!(window.get_title(), String::from("Direct Game Engine Window")); // The title of the window is "Direct Game Engine Window" by default.
    /// ```
    pub fn get_title(&self) -> String {
        return self.title.clone();
    }

    #[allow(dead_code)]
    pub(crate) fn title(&self) -> *const u16 {
        return wchar(&self.title);
    }

    /// Returns the position of the window
    ///
    /// # Example
    ///
    /// ```ignore
    /// let window = Window::default();
    /// assert_eq!(window.get_position().x, 0); assert_eq!(window.get_position().y, 0);
    /// ```
    pub fn get_pos(&self) -> Point {
        return self.pos;
    }

    /// Return the x position of the window
    ///
    /// # Example
    ///
    /// ```ignore
    /// let window = Window::default();
    /// assert_eq!(window.get_x(), 0); // The x position of the window is 0i32 by default.
    /// ```
    pub fn get_x(&self) -> i32 {
        return self.pos.x;
    }

    /// Return the y position of the window
    ///
    /// # Example
    ///
    /// ```ignore
    /// let window = Window::default();
    /// assert_eq!(window.get_y(), 0); // The y position of the window is 0i32 by default.           
    /// ```
    pub fn get_y(&self) -> i32 {
        return self.pos.y;
    }

    /// Returns the size of the window
    ///
    /// # Example
    ///
    /// ```ignore
    /// let window = Window::default();
    /// assert_eq!(window.get_size().x, 800); assert_eq!(window.get_size().height, 640);
    /// ```
    pub fn get_size(&self) -> Size {
        return self.size;
    }

    /// Returns the width of the window
    ///
    /// # Example
    ///
    /// ```ignore
    /// let window = Window::default();
    /// assert_eq!(window.get_width(), 800); // The width of the window is 800i32 by default.
    /// ```
    pub fn get_width(&self) -> i32 {
        return self.size.width;
    }

    /// Returns the height of the window
    ///
    /// # Example
    ///
    /// ```ignore
    /// let window = Window::default();
    /// assert_eq!(window.get_height(), 640); // The height of the window is 600i32 by default
    /// ```
    pub fn get_height(&self) -> i32 {
        return self.size.height;
    }

    /// Creates a new window with the given window handle
    ///
    /// # Example
    ///
    /// ```ignore
    /// let hwnd = CreateWindowExW(0, "class", "Direct Game Engine Window", WS_OVERLAPPEDWINDOW, CW_USEDEFAULT, CW_USEDEFAULT, 800, 640, null_mut(), null_mut(), null_mut());
    /// let window = window::from(hwnd);
    /// ```
    pub(crate) fn from(hwnd: HWND) -> Self {
        let title = unsafe {
            let mut title = Vec::with_capacity(GetWindowTextLengthW(hwnd) as usize + 1usize);
            GetWindowTextW(hwnd, title.as_mut_ptr(), title.capacity() as i32);
            String::from_utf16_lossy(&title)
        };

        let (x, y, w, h) = unsafe {
            let mut wr = std::mem::zeroed();
            GetClientRect(hwnd, &mut wr);
            (wr.left, wr.top, wr.right - wr.left, wr.bottom - wr.top)
        };

        return Self {
            hwnd,
            title,
            pos: Point::new(x, y),
            size: Size::new(w, h),
        };
    }

    /// Retrieves the class of the window
    ///
    /// # Example
    ///
    /// ```ignore
    /// let window = Window::default();
    /// assert_eq!(window.get_class_name(), String::new()); // Window class is empty String by default.
    /// ```
    pub fn get_class_name(&self) -> String {
        unsafe {
            let mut class = Vec::with_capacity(512);
            let size = GetClassNameW(self.hwnd, class.as_mut_ptr(), 512);
            class.set_len(size as usize);
            String::from_utf16_lossy(&class)
        }
    }

    pub fn get_hwnd_class_name(hwnd: HWND) -> String {
        unsafe {
            let mut class = Vec::with_capacity(512);
            let size = GetClassNameW(hwnd, class.as_mut_ptr(), 512);
            class.set_len(size as usize);
            String::from_utf16_lossy(&class)
        }
    }

    pub(crate) unsafe fn register<T>(
        class: &str,
        builder: WindowBuilder,
        data: *const T,
        callback: unsafe extern "system" fn(HWND, UINT, WPARAM, LPARAM) -> LRESULT,
    ) -> HWND {
        let wc = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: 0u32,
            lpfnWndProc: Some(callback),
            cbClsExtra: 0i32,
            cbWndExtra: 0i32,
            hInstance: GetModuleHandleW(std::ptr::null_mut()),
            hIcon: load_icon(&builder.icon).cast(),
            hCursor: std::ptr::null_mut(),
            hbrBackground: std::ptr::null_mut(),
            lpszMenuName: std::ptr::null_mut(),
            lpszClassName: wchar(class),
            hIconSm: load_icon(&builder.icon).cast(),
        };

        RegisterClassExW(&wc);

        let mut wr: RECT = std::mem::zeroed();
        wr.left = 100i32;
        wr.top = 100i32;
        wr.right = wr.left + builder.get_width();
        wr.bottom = wr.top + builder.get_height();

        let mut style = WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;
        if builder.resizable {
            style |= WS_THICKFRAME;
        }
        AdjustWindowRect(&mut wr, style, FALSE);
        let hwnd = CreateWindowExW(
            0u32,
            wchar(class),
            wchar(&builder.title),
            style,
            builder.get_x(),
            builder.get_y(),
            wr.right - wr.left,
            wr.bottom - wr.top,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            data as *mut c_void,
        );

        let (darkmode, value) = match builder.get_theme() {
            Theme::Dark => (20, TRUE),
            Theme::Light => (0, FALSE),
        };

        DwmSetWindowAttribute(
            hwnd,
            darkmode,
            &value as *const BOOL as *const c_void,
            std::mem::size_of_val(&value) as DWORD,
        );

        ShowWindow(hwnd, SW_SHOW);

        return hwnd;
    }
}

pub use raw_window_handle::{HasRawWindowHandle, RawWindowHandle, Win32WindowHandle};

unsafe impl HasRawWindowHandle for Window {
    fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        let mut hwnd = Win32WindowHandle::empty();
        hwnd.hwnd = self.hwnd.cast();
        hwnd.hinstance = unsafe { GetModuleHandleW(std::ptr::null()).cast() };

        RawWindowHandle::Win32(hwnd)
    }
}
