use winapi::{
    ctypes::*,
    shared::{minwindef::*, windef::*},
    um::{dwmapi::DwmSetWindowAttribute, libloaderapi::*, winuser::*},
};

use crate::prelude::*;

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
    pub fn new(id: usize) -> Self {
        let mut window: Self = Self::default();
        window.hwnd = id as HWND;
        return window;
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_owned();
        unsafe {
            SetWindowTextW(self.hwnd, wchar(title));
        }
    }

    pub fn set_icon(&mut self, path: &str) {
        unsafe {
            SetClassLongPtrW(self.hwnd, GCLP_HICON, load_icon(path) as isize);
        }
    }

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

    pub fn get_id(&self) -> usize {
        return self.hwnd as usize;
    }

    pub fn get_title(&self) -> String {
        return self.title.clone();
    }

    #[allow(dead_code)]
    pub(crate) fn title(&self) -> *const u16 {
        return wchar(&self.title);
    }

    pub fn get_pos(&self) -> Point {
        return self.pos;
    }

    pub fn get_x(&self) -> i32 {
        return self.pos.x;
    }

    pub fn get_y(&self) -> i32 {
        return self.pos.y;
    }

    pub fn get_size(&self) -> Size {
        return self.size;
    }

    pub fn get_width(&self) -> i32 {
        return self.size.width;
    }

    pub fn get_height(&self) -> i32 {
        return self.size.height;
    }

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

    pub fn get_class_name(&self) -> String {
        unsafe {
            let mut class = Vec::with_capacity(512);
            let size = GetClassNameW(self.hwnd, class.as_mut_ptr(), 512);
            class.set_len(size as usize);
            String::from_utf16_lossy(&class)
        }
    }

    pub(crate) unsafe fn register<T>(
        class: &str,
        builder: WindowBuilder,
        data: *mut T,
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
