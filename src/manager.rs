#![allow(dead_code, unused_unsafe, unused_imports)]

use std::{
    collections::HashMap,
    sync::mpsc::{Receiver, Sender},
};

use winapi::{
    ctypes::*,
    shared::{minwindef::*, windef::*},
    um::{wingdi::MAKEPOINTS, winuser::*},
};

use crate::prelude::*;

/// Central point of this crate. A Manager processes the events and messages of every window. It gives some miscellaneous information as well such as the time.
/// 
/// # Example
/// 
/// ```ignore
/// let mut manager = Manager::new(WindowBuilder::default());
/// 
/// manager.run(|events, control_flow, manager| {
///     match events => {
///         Events::WindowEvent { id, event } match event {
///             WindowEvents::Close => {
///                 if id == manager.window().get_id() {
///                     *control_flow = ControlFlow::Exit;
///                 }
///             },
///             _=> {}
///         }
///         _=> {}
///     }
/// 
///     if manager.get_key(ESCAPE) == Action::Press {
///         *control_flow = ControlFlow::Exit;
///     }
/// });
/// ```
#[derive(Debug)]
pub struct Manager {
    windows: HashMap<String, Window>,
    builders: HashMap<String, WindowBuilder>,
    mouse: Mouse,
    keyboard: Keyboard,
    timer: Timer,
    sender: Sender<Events>,
    receiver: Receiver<Events>,
    close: bool,
}

impl Default for Manager {
    fn default() -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();
        return Self {
            mouse: Mouse::new(),
            timer: Timer::new(),
            keyboard: Keyboard::new(false),
            windows: HashMap::default(),
            builders: HashMap::default(),
            sender,
            receiver,
            close: false,
        };
    }
}

impl Manager {
    #[allow(non_upper_case_globals)]
    const DGEWindowClassExWName: &'static str = "DGEWindowClassExWName";

    /// Creates a new instance of the Window Manager. Unlikely to default(), you have to give one WindowBuilder sturct.
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// let manager = Manager::new(WindowBuilder::default()
    ///     .with_title("Hello, World!")
    ///     .with_dimensions(800, 640)
    ///     .with_pos(60, 6)
    ///     .with_icon("path\\to\\your\\icon\\.ico"));
    /// ```
    pub fn new(builder: WindowBuilder) -> Self {
        let mut builders = HashMap::new();
        builders.insert(Self::DGEWindowClassExWName.to_string(), builder);
        return Self {
            builders,
            ..Default::default()
        };
    }

    /// Inserts a new window. You have to give each new extra window a class which is basically the same as 'key' in HashMap<T>. There should be no white spaces.
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// let manager = Manager::new(WindowBuilder::default())
    ///     .ass_window("MyWindow", WindowBuilder::default()
    ///         .with_title("My Own new Extra WiNdOw")
    ///         .with_theme(Theme::Dark));
    ///     .ass_window("AnotherWindow", WindowBuilder::default()
    ///         .with_title("Another tiny one as well")
    ///         .with_dimensions(60, 60));
    /// ```
    pub fn add_window(mut self, class: &str, builder: WindowBuilder) -> Self {
        self.builders.insert(class.to_string(), builder);
        return self;
    }

    /// Returns a reference to the default window of the manager
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// let window = manager.window();
    /// println!("A window with id: {} has title of {}", window.get_id(), window.get_title());
    /// ```
    pub fn window(&self) -> &Window {
        return self.get_window(Self::DGEWindowClassExWName);
    }

    /// Returns a reference to the mut default window of the manager
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// let window = manager.mut_window();
    /// println!("A window with id: {} has title of {}", window.get_id(), window.get_title());
    /// 
    /// window.set_title("New Title");
    /// println!("A window with id: {} has title of {}", window.get_id(), window.get_title());
    /// ```
    pub fn mut_window(&mut self) -> &mut Window {
        return self.get_mut_window(Self::DGEWindowClassExWName);
    }

    /// Returns a reference to a window with a specified class. Panics if there is no a window with that class!
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// let window = manager.get_window("YourWindowClassName");
    /// println!("A window with id: {} has title of {}", window.get_id(), window.get_title());
    /// ```
    pub fn get_window(&self, class: &str) -> &Window {
        return self.windows.get(class).unwrap();
    }

    /// Returns a reference to a mut window with a specified class. Panics if there is no a window with that class!
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// let window = manager.get_mut_window("YourWindowClassName");
    /// println!("A window with id: {} has title of {}", window.get_id(), window.get_title());
    /// 
    /// window.set_title("New Title");
    /// println!("A window with id: {} has title of {}", window.get_id(), window.get_title());
    /// ```
    pub fn get_mut_window(&mut self, class: &str) -> &mut Window {
        return self.windows.get_mut(class).unwrap();
    }

    /// Runs the program. This function takes a closure as its parameter which then gives back events, control flow and manager itself. The windows are processed in their own threads outside the main thread so that your program will not wait until the events are finished.
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// let mut manager = Manager::new(WindowBuilder::default());
    /// 
    /// manager.run(|events, control_flow, manager| {
    ///     match events {
    ///         Events::WindowEvent { id, event } => match event {
    ///             WindowEvents::Close => if id == manager.window().get_id() => *control_flow = ControlFlow::Exit,
    ///             _=> {}
    ///         }
    ///         _=> {}
    ///     }
    /// });
    /// ```
    pub fn run<T>(&mut self, mut func: T)
    where
        T: FnMut(Events, &mut ControlFlow, &mut Manager),
    {
        self.spawn_window_thread();

        let mut control_flow = ControlFlow::default();

        'user_events_loop: loop {
            while let Ok(events) = self.try_recv() {
                func(events, &mut control_flow, self);
                match control_flow {
                    ControlFlow::Continue => {},
                    ControlFlow::Exit => {
                        self.close = true;
                        break 'user_events_loop;
                    }
                    ControlFlow::ExitWithCode(exit_code) => {
                        panic!("Exit code with {}", exit_code);
                    }
                }
            }

            func(
                Events::default(),
                &mut control_flow,
                self,
            );
            match control_flow {
                ControlFlow::Continue => {},
                ControlFlow::Exit => {
                    self.close = true;
                    break 'user_events_loop;
                }
                ControlFlow::ExitWithCode(exit_code) => {
                    panic!("Exit with code {}", exit_code);
                }
            }

            control_flow = ControlFlow::Continue;
        }
    }

    fn spawn_window_thread(&mut self) {
        let thread_builders = self.builders.clone();
        let thread_data = self as *mut Self as usize;
        std::thread::spawn(move || unsafe {
            for (class, builder) in thread_builders {
                Window::register(&class, builder, thread_data as *mut Self, Self::setup);
            }
            let manager = (thread_data as *mut Self).as_mut().unwrap();

            let mut msg = std::mem::zeroed();
            loop {
                manager.keyboard.clear();
                manager.mouse.clear_keystates();

                if PeekMessageW(&mut msg, 0 as HWND, 0, 0, PM_REMOVE) > 0 {
                    TranslateMessage(&msg);
                    DispatchMessageW(&msg);

                    if manager.close {
                        break;
                    }
                }
            }
        });
    }

    unsafe fn wndproc(&mut self, hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        match msg {
            WM_DESTROY => {
                self.send(Events::WindowEvent {
                    id: hwnd as usize,
                    event: WindowEvents::Close,
                });
                PostQuitMessage(0);
            }

            WM_MOUSEMOVE => {
                let x = MAKEPOINTS(lparam as u32).x;
                let y = MAKEPOINTS(lparam as u32).y;

                self.mouse.update_pos(x, y);
                self.send(Events::MouseEvent {
                    id: hwnd as usize,
                    event: MouseEvents::MouseMove {
                        x: self.mouse.x(),
                        y: self.mouse.y(),
                        last_x: self.mouse.last_x(),
                        last_y: self.mouse.last_y(),
                        dx: self.mouse.x_offset(),
                        dy: self.mouse.y_offset(),
                    },
                });
            }

            WM_MOUSEWHEEL => {
                let delta = GET_WHEEL_DELTA_WPARAM(wparam);
                self.send(Events::MouseEvent {
                    id: hwnd as usize,
                    event: MouseEvents::Scroll {
                        y_offset: delta / WHEEL_DELTA,
                    },
                });
            }

            WM_CHAR => {
                self.keyboard.set_is_char(wparam, true);
                self.send(Events::KeyboardEvent {
                    id: hwnd as usize,
                    event: KeyboardEvents::Char {
                        keycode: wparam,
                    },
                });
            }

            WM_KEYDOWN | WM_SYSKEYDOWN => {
                self.keyboard.set_is_down(wparam, true);
                self.keyboard
                    .set_is_changed(wparam, (lparam & (1 << 30)) == 0);

                if !self.keyboard.is_changed(wparam) {
                    self.send(Events::KeyboardEvent {
                        id: hwnd as usize,
                        event: KeyboardEvents::Key {
                            keycode: wparam,
                            action: Action::Down,
                        },
                    });
                } else if self.keyboard.is_released(wparam) {
                    self.keyboard.set_is_released(wparam, false);
                    self.send(Events::KeyboardEvent {
                        id: hwnd as usize,
                        event: KeyboardEvents::Key {
                            keycode: wparam,
                            action: Action::Press,
                        },
                    });
                }
            }

            WM_KEYUP | WM_SYSKEYUP => {
                self.keyboard.set_is_down(wparam, false);
                self.keyboard.set_is_changed(wparam, true);
                self.keyboard.set_is_released(wparam, true);
                self.send(Events::KeyboardEvent {
                    id: hwnd as usize,
                    event: KeyboardEvents::Key {
                        keycode: wparam,
                        action: Action::Release,
                    },
                });
            }

            WM_SIZE => {
                let width = LOWORD(lparam as u32) as i32;
                let height = HIWORD(lparam as u32) as i32;
                if wparam == SIZE_MAXIMIZED {
                    self.send(Events::WindowEvent {
                        id: hwnd as usize,
                        event: WindowEvents::Maximized { width, height },
                    });
                } else if wparam == SIZE_MINIMIZED {
                    self.send(Events::WindowEvent {
                        id: hwnd as usize,
                        event: WindowEvents::Minimized { width, height },
                    });
                } else {
                    self.send(Events::WindowEvent {
                        id: hwnd as usize,
                        event: WindowEvents::FramebufferChanged { width, height },
                    });
                }
            }

            WM_MOVE => {
                let x = LOWORD(lparam as u32) as i32;
                let y = HIWORD(lparam as u32) as i32;
                self.send(Events::WindowEvent {
                    id: hwnd as usize,
                    event: WindowEvents::Moved { x, y },
                });
            }

            WM_LBUTTONDOWN => {
                let x = LOWORD(lparam as u32) as i32;
                let y = HIWORD(lparam as u32) as i32;
                self.mouse.set_l_button_down(true);
                self.mouse.set_l_button_released(false);
                self.mouse.set_l_button_changed(true);

                self.send(Events::MouseEvent { 
                    id: hwnd as usize,
                    event: MouseEvents::LButton { action: Action::Press, pos: Point::new(x, y) },
                });
            }

            WM_LBUTTONUP => {
                let x = LOWORD(lparam as u32) as i32;
                let y = HIWORD(lparam as u32) as i32;
                self.mouse.set_l_button_down(false);
                self.mouse.set_l_button_released(true);
                self.mouse.set_l_button_changed(true);

                self.send(Events::MouseEvent { 
                    id: hwnd as usize,
                    event: MouseEvents::LButton { action: Action::Release, pos: Point::new(x, y) },
                });
            }

            WM_RBUTTONDOWN => {
                let x = LOWORD(lparam as u32) as i32;
                let y = HIWORD(lparam as u32) as i32;
                self.mouse.set_r_button_down(true);
                self.mouse.set_r_button_released(false);
                self.mouse.set_r_button_changed(true);

                self.send(Events::MouseEvent { 
                    id: hwnd as usize,
                    event: MouseEvents::RButton { action: Action::Press, pos: Point::new(x, y) },
                });
            }

            WM_RBUTTONUP => {
                let x = LOWORD(lparam as u32) as i32;
                let y = HIWORD(lparam as u32) as i32;
                self.mouse.set_r_button_down(false);
                self.mouse.set_r_button_released(true);
                self.mouse.set_r_button_changed(true);

                self.send(Events::MouseEvent { 
                    id: hwnd as usize,
                    event: MouseEvents::RButton { action: Action::Release, pos: Point::new(x, y) },
                });
            }

            WM_MBUTTONDOWN => {
                let x = LOWORD(lparam as u32) as i32;
                let y = HIWORD(lparam as u32) as i32;
                self.mouse.set_m_button_down(true);
                self.mouse.set_m_button_released(false);
                self.mouse.set_m_button_changed(true);

                self.send(Events::MouseEvent { 
                    id: hwnd as usize,
                    event: MouseEvents::MButton { action: Action::Press, pos: Point::new(x, y) },
                });
            }

            WM_MBUTTONUP => {
                let x = LOWORD(lparam as u32) as i32;
                let y = HIWORD(lparam as u32) as i32;
                self.mouse.set_m_button_down(false);
                self.mouse.set_m_button_released(true);
                self.mouse.set_m_button_changed(true);

                self.send(Events::MouseEvent { 
                    id: hwnd as usize,
                    event: MouseEvents::MButton { action: Action::Release, pos: Point::new(x, y) },
                });
            }

            WM_XBUTTONDOWN => {
                let x = LOWORD(lparam as u32) as i32;
                let y = HIWORD(lparam as u32) as i32;

                if LOWORD(wparam as u32) as usize & MK_XBUTTON1 > 0 {
                    self.mouse.set_x1_button_down(true);
                    self.mouse.set_x1_button_released(false);
                    self.mouse.set_x1_button_changed(true);
    
                    self.send(Events::MouseEvent { 
                        id: hwnd as usize,
                        event: MouseEvents::X1Button { action: Action::Press, pos: Point::new(x, y) },
                    });
                } else if LOWORD(wparam as u32) as usize & MK_XBUTTON2 > 0 {
                    self.mouse.set_x2_button_down(true);
                    self.mouse.set_x2_button_released(false);
                    self.mouse.set_x2_button_changed(true);
    
                    self.send(Events::MouseEvent { 
                        id: hwnd as usize,
                        event: MouseEvents::X2Button { action: Action::Press, pos: Point::new(x, y) },
                    });
                }
            }

            WM_XBUTTONUP => {
                let x = LOWORD(lparam as u32) as i32;
                let y = HIWORD(lparam as u32) as i32;
                
                if HIWORD(wparam as u32) & XBUTTON1 > 0 {
                    self.mouse.set_x1_button_down(false);
                    self.mouse.set_x1_button_released(true);
                    self.mouse.set_x1_button_changed(true);
    
                    self.send(Events::MouseEvent { 
                        id: hwnd as usize,
                        event: MouseEvents::X1Button { action: Action::Release, pos: Point::new(x, y) },
                    });
                } else if HIWORD(wparam as u32) & XBUTTON2 > 0 {
                    self.mouse.set_x2_button_down(false);
                    self.mouse.set_x2_button_released(true);
                    self.mouse.set_x2_button_changed(true);
    
                    self.send(Events::MouseEvent { 
                        id: hwnd as usize,
                        event: MouseEvents::X2Button { action: Action::Release, pos: Point::new(x, y) },
                    });
                }
            }

            WM_SETFOCUS => {
                self.send(Events::WindowEvent {
                    id: hwnd as usize,
                    event: WindowEvents::SetFocus
                });
            }

            WM_KILLFOCUS => {
                self.send(Events::WindowEvent {
                    id: hwnd as usize,
                    event: WindowEvents::LostFocus
                });
            }

            _ => {
                self.send(Events::default());
            }
        }

        return DefWindowProcW(hwnd, msg, wparam, lparam);
    }

    unsafe extern "system" fn process_messages(
        hwnd: HWND,
        msg: UINT,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        return (GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut Self)
            .as_mut()
            .unwrap()
            .wndproc(hwnd, msg, wparam, lparam);
    }

    unsafe extern "system" fn setup(
        hwnd: HWND,
        msg: UINT,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        if msg == WM_CREATE {
            let create_struct = lparam as *mut CREATESTRUCTW;
            let manager_ptr = create_struct.as_ref().unwrap().lpCreateParams as *mut Self;
            let manager = manager_ptr.as_mut().unwrap();
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, manager_ptr as isize);
            SetWindowLongPtrW(hwnd, GWLP_WNDPROC, Self::process_messages as isize);
            manager.send(Events::WindowEvent {
                id: hwnd as usize,
                event: WindowEvents::Create,
            });
            let wnd = Window::from(hwnd);
            manager.windows.insert(wnd.get_class_name(), wnd);
            return manager.wndproc(hwnd, msg, wparam, lparam);
        }

        return DefWindowProcW(hwnd, msg, wparam, lparam);
    }

    /// Retrieves typed Keyboard buttons and keys. (Case sensitie!)
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// if manager.get_char(u) {
    ///     println!("small case u char is typed");
    /// } else if manager.get_char(T) {
    ///     println!("capital case T char is typed");
    /// }
    /// ```
    pub fn get_char(&self, char: usize) -> bool {
        return self.keyboard.is_char(char);
    }

    /// Retrieves the state of the Keyboard buttons and keys. (Not case sensitive!)
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// if manager.get_key(TAB) == Action::Down {
    ///     println!("Tabbar is being pressed!");
    /// }
    /// ```
    pub fn get_key(&self, keycode: usize) -> Action {
        return if self.keyboard.is_down(keycode) && !self.keyboard.is_changed(keycode) {
            Action::Down
        } else if self.keyboard.is_down(keycode) && self.keyboard.is_changed(keycode) {
            Action::Press
        } else if !self.keyboard.is_down(keycode) && self.keyboard.is_changed(keycode) {
            Action::Release
        } else {
            Action::None
        };
    }

    /// Retrieves the state of the mouse buttons
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// if manager.get_mouse_button(Button::MBUTTON) == Action::Press {
    ///     *control_flow = ControlFlow::Exit;
    /// } 
    /// ```
    pub fn get_mouse_button(&self, button: usize) -> Action {
        match button {
            Button::LBUTTON => { 
                if self.mouse.l_button().is_changed() && self.mouse.l_button.is_down() {
                    return Action::Press;
                } else if !self.mouse.l_button().is_changed() && self.mouse.l_button.is_down() {
                    return Action::Down;
                } else if !self.mouse.l_button().is_down() && self.mouse.l_button().is_changed() {
                    return Action::Release;
                } else {
                    return Action::None;
                }
            }

            Button::RBUTTON => { 
                if self.mouse.r_button().is_changed() && self.mouse.r_button.is_down() {
                    return Action::Press;
                } else if !self.mouse.r_button().is_changed() && self.mouse.r_button.is_down() {
                    return Action::Down;
                } else if !self.mouse.r_button().is_down() && self.mouse.r_button().is_changed() {
                    return Action::Release;
                } else {
                    return Action::None;
                }
            }

            Button::MBUTTON => { 
                if self.mouse.m_button().is_changed() && self.mouse.m_button.is_down() {
                    return Action::Press;
                } else if !self.mouse.m_button().is_changed() && self.mouse.m_button.is_down() {
                    return Action::Down;
                } else if !self.mouse.m_button().is_down() && self.mouse.m_button().is_changed() {
                    return Action::Release;
                } else {
                    return Action::None;
                }
            }

            Button::XBUTTON1 => { 
                if self.mouse.x1_button().is_changed() && self.mouse.x1_button.is_down() {
                    return Action::Press;
                } else if !self.mouse.x1_button().is_changed() && self.mouse.x1_button.is_down() {
                    return Action::Down;
                } else if !self.mouse.x1_button().is_down() && self.mouse.x1_button().is_changed() {
                    return Action::Release;
                } else {
                    return Action::None;
                }
            }
            
            Button::XBUTTON2 => { 
                if self.mouse.x2_button().is_changed() && self.mouse.x2_button.is_down() {
                    return Action::Press;
                } else if !self.mouse.x2_button().is_changed() && self.mouse.x2_button.is_down() {
                    return Action::Down;
                } else if !self.mouse.x2_button().is_down() && self.mouse.x2_button().is_changed() {
                    return Action::Release;
                } else {
                    return Action::None;
                }
            }

            _=> {
                panic!("There is no virtual mouse button code like {button}");
            }
        }
    }

    fn send(&self, events: Events) {
        self.sender.send(events).unwrap();
    }

    fn try_recv(&self) -> Result<Events, std::sync::mpsc::TryRecvError> {
        return self.receiver.try_recv();
    }

    /// Retrieves the delta time and current frame
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// println!("Time is {}", manager.time().1);
    /// ```
    pub fn time(&mut self) -> (f32, f32) {
        return (self.timer.dt(), self.timer.current_frame);
    }
}
