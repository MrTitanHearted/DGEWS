use crate::common::Point;

/// The state of the buttons such as being pressed or released or none as well
/// 
/// # Example
/// 
/// ```ignore
/// if manager.get_key(A) == Action::Release {
///     println!("A key is released");
/// }
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Action {
    /// When a key is pressed
    Press,
    /// When a key is released
    Release,
    /// When a key is down
    Down,
    /// When a key is idle
    None,
}

/// Main central Events that are sent by the window or windows via Manager sturct
/// 
/// # Example
/// 
/// ```ignore
/// let mut manager = Manager::new(WindowBuilder::default());
/// 
/// manager.run(|events, control_flow, _| {
///     match events => {
///         Events::WindowEvent { id, event } match event {
///             WindowEvents::Close => *control_flow = ControlFlow::Exit,
///             WindowEvents::Create => {
///                 println!("A window is created with id: {id}");
///             }
///             _=> {}
///         }
///         _=> {}
///     }
/// });
/// ```
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub enum Events {
    /// WindowEvents such as moving window or changing the size
    WindowEvents { id: usize, event: WindowEvents },
    /// KeyboardEvents like pressing Left Shift button
    KeyboardEvents { id: usize, event: KeyboardEvents },
    /// MouseEvents. For example, releasing Right Mouse Button or scrolling up and down
    MouseEvents { id: usize, event: MouseEvents },
    /// Idle form which means nothing is happening
    #[default]
    None,
}

/// Specific window events
/// 
/// # Example
/// 
/// ```ignore
/// let mut manager = Manager::new(WindowBuilder::default());
/// 
/// manager.run(|events, control_flow, _| {
///     match events => {
///         Events::WindowEvent { id, event } match event {
///             WindowEvents::Close => *control_flow = ControlFlow::Exit,
///             WindowEvents::SetFocus => {
///                 println!("A window with id: {id} has gained the focus");
///             }   
///             WindowEvents::LostFocus => {
///                 println!("A window with id: {id} has lost the focus");
///             }   
///             _=> {}
///         }
///         _=> {}
///     }
/// });
/// ```
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum WindowEvents {
    /// Sent when a window is created
    Create,
    /// Sent when a window is closed
    Close,
    /// Sent when a window is maximized
    Maximized { width: i32, height: i32 },
    /// Sent when a window is minimized
    Minimized { width: i32, height: i32 },
    /// Sent when a window size is changed
    FramebufferChanged { width: i32, height: i32 },
    /// Sent when a window is moved
    Moved { x: i32, y: i32 },
    /// Sent when a window has gained the focus
    SetFocus,
    /// Sent when a window has lost the focus
    LostFocus,
}

/// Specific keyboard events
/// 
/// # Exapmle
/// 
/// ```
/// let mut manager = Manager::new(WindowBuilder::default());
/// 
/// manager.run(|events, control_flow, _| {
///     match events => {
///         Events::WindowEvent { id, event } match event {
///             WindowEvents::Close => *control_flow = ControlFlow::Exit,
///             _=> {}
///         }
///         Events::KeyboardEvent { id:_, event } match event {
///             KeyboardEvents::Key { keycode, action } => {
///                 if keycode == SPACE && action == Action::Down {
///                     println!("Space bar is being down");
///                 }
///             }
///             _=> {}
///         }
///         _=> {}
///     }
/// });
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeyboardEvents {
    /// Sent when a key or a button is pressed, released or down
    Key { keycode: usize, action: Action },
    /// Sent when a character is pressed. Difference between Key and Char events is that Char event is sensitive to the case of that key, while Key event is not!
    Char { keycode: usize },
}

/// Specific mouse events
/// 
/// # Example
/// 
/// ```ignore
/// let mut manager = Manager::new(WindowBuilder::default());
/// 
/// manager.run(|events, control_flow, _| {
///     match events => {
///         Events::WindowEvent { id, event } match event {
///             WindowEvents::Close => *control_flow = ControlFlow::Exit,
///             _=> {}
///         }
///         Events::MouseEvent { id, event } match event {
///             MouseEvents::MouseMove { x, y, last_x, last_y, _, _ } => {
///                 println!("Cursor has moved from point ({last_x}, {last_y}) to ({x}, {y}) in the window with id: {id}");
///             }
///             _=> {}
///         }
///         _=> {}
///     }
/// });
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MouseEvents {
    /// Sent when a mouse is scrolling up or down
    Scroll { y_offset: i16 },
    /// Sent when a left mouse button is pressed, released or down
    LButton { action: Action, pos: Point },
    /// Sent when a right mouse button is pressed, released or down
    RButton { action: Action, pos: Point },
    /// Sent when a middle mouse button is pressed, released or down
    MButton { action: Action, pos: Point },
    /// Sent when a x mouse button 1 is pressed, released or down
    X1Button { action: Action, pos: Point },
    /// Sent when a x mouse button 2 is pressed, released or down
    X2Button { action: Action, pos: Point },
    /// Sent when a cursor is moved from one point to another where x is new x position, y is new y position, last_x is last x position, last_y is last y position, dx is delta x (x - last_x) and dy is delta y (y - last_y)
    MouseMove { x: i16, y: i16, last_x: i16, last_y: i16, dx: i16, dy: i16 },
}

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub(crate) enum MainEvents {
    MainWindowEvent { id: usize, event: MainWindowEvents },
    MainKeyboardEvent { id: usize, event: MainKeyboardEvents },
    MainMouseEvent { id: usize, event: MainMouseEvents },
    #[default]
    None,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum MainWindowEvents {
    Create,
    Close,
    Maximized { width: i32, height: i32 },
    Minimized { width: i32, height: i32 },
    FramebufferChanged { width: i32, height: i32 },
    Moved { x: i32, y: i32 },
    SetFocus,
    LostFocus,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum MainKeyboardEvents {
    Key { up: bool, is_changed: bool, keycode: usize },
    Char { keycode: usize },
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum MainMouseEvents {
    Scroll { y_offset: i16 },
    LButton { up: bool, pos: Point },
    RButton { up: bool, pos: Point },
    MButton { up: bool, pos: Point },
    XButton { up: bool, wparam: u32, pos: Point },
    MouseMove { x: i16, y: i16 },
}