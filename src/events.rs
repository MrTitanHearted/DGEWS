use crate::common::Point;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Action {
    Press,
    Release,
    Down,
    None,
}

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub enum Events {
    WindowEvent { id: usize, event: WindowEvents }, 
    KeyboardEvent { id: usize, event: KeyboardEvents },
    MouseEvent { id: usize, event: MouseEvents },
    #[default]
    None,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum WindowEvents {
    Create,
    Close,
    Maximized { width: i32, height: i32 },
    Minimized { width: i32, height: i32 },
    Moved { x: i32, y: i32 },
    FramebufferChanged { width: i32, height: i32 },
    SetFocus,
    LostFocus,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeyboardEvents {
    Key { keycode: usize, action: Action },
    Char { keycode: usize },
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MouseEvents {
    Scroll { y_offset: i16 },
    LButton { action: Action, pos: Point },
    RButton { action: Action, pos: Point },
    MButton { action: Action, pos: Point },
    X1Button { action: Action, pos: Point },
    X2Button { action: Action, pos: Point },
    MouseMove { x: i16, y: i16, last_x: i16, last_y: i16, dx: i16, dy: i16 },
}