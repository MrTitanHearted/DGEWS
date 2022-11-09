#![allow(dead_code, unused_unsafe, unused_imports)]

use std::{
    collections::HashMap,
    sync::{
        mpsc::{Receiver, Sender},
        Arc, Mutex,
    },
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
    mouse: Mouse,
    keyboard: Keyboard,
    timer: Timer,
    msger: Messenger,
    close: bool,
    sender: Sender<Events>,
    receiver: Receiver<Events>,
}

impl Default for Manager {
    fn default() -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();
        return Self {
            mouse: Mouse::new(),
            timer: Timer::new(),
            keyboard: Keyboard::new(false),
            windows: HashMap::default(),
            msger: Messenger::new(),
            close: false,
            sender,
            receiver,
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
        let mut manager = Manager::default();
        manager.insert(Self::DGEWindowClassExWName, builder);
        return manager;
    }

    /// Inserts a new window. You have to give each new extra window a class which is basically the same as 'key' in HashMap<T>. There should be no white spaces.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let manager = Manager::new(WindowBuilder::default())
    ///     .add_window("MyWindow", WindowBuilder::default()
    ///         .with_title("My Own new Extra WiNdOw")
    ///         .with_theme(Theme::Dark));
    ///     .add_window("AnotherWindow", WindowBuilder::default()
    ///         .with_title("Another tiny one as well")
    ///         .with_dimensions(60, 60));
    /// ```
    pub fn add_window(mut self, class: &str, builder: WindowBuilder) -> Self {
        self.insert(class, builder);
        return self;
    }

    fn insert(&mut self, class: &str, builder: WindowBuilder) {
        let msger = self.msger.clone();
        let class = class.to_string();
        let mut added = false;
        let mut hwnd: *mut HWND__ = std::ptr::null_mut();

        let p_added = &mut added as *mut bool as usize;
        let p_hwnd = &mut hwnd as *mut HWND as usize;

        std::thread::spawn(move || unsafe {
            let window = Window::register(&class, builder, &msger as *const Messenger, Self::setup);

            let hwnd = (p_hwnd as *mut HWND).as_mut().unwrap();
            *hwnd = window;

            let not_added = (p_added as *mut bool).as_mut().unwrap();
            *not_added = true;

            let mut msg = std::mem::zeroed();
            loop {
                if PeekMessageW(&mut msg, window, 0, 0, PM_REMOVE) > 0 {
                    TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }
            }
        });

        'window: loop {
            if added {
                let window = Window::from(hwnd as *mut _);
                self.windows.insert(window.get_class_name(), window);

                break 'window;
            }
        }
    }

    /// Returns a reference to the default window of the manager
    ///
    /// # Example
    ///
    /// ```ignore
    /// let window = manager.window();
    /// println!("A window with id: {} has title of {}", window.get_id(), window.get_title());
    /// ```
    pub fn window(&self) -> Option<&Window> {
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
    pub fn mut_window(&mut self) -> Option<&mut Window> {
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
    pub fn get_window(&self, class: &str) -> Option<&Window> {
        return self.windows.get(class);
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
    pub fn get_mut_window(&mut self, class: &str) -> Option<&mut Window> {
        return self.windows.get_mut(class);
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
        let mut control_flow = ControlFlow::default();

        'user_events_loop: loop {
            while let Ok(main_events) = self.msger.try_recv() {
                self.keyboard.clear();
                self.mouse.clear_keystates();

                let events = match main_events {
                    MainEvents::MainWindowEvent { id, event } => match event {
                        MainWindowEvents::Create => Events::WindowEvents {
                            id,
                            event: WindowEvents::Create,
                        },
                        MainWindowEvents::Close => {
                            let class = Window::get_hwnd_class_name(id as *mut _);
                            self.windows.remove(&class);

                            Events::WindowEvents {
                                id,
                                event: WindowEvents::Close,
                            }
                        }
                        MainWindowEvents::Maximized { width, height } => Events::WindowEvents {
                            id,
                            event: WindowEvents::Maximized { width, height },
                        },
                        MainWindowEvents::Minimized { width, height } => Events::WindowEvents {
                            id,
                            event: WindowEvents::Minimized { width, height },
                        },
                        MainWindowEvents::FramebufferChanged { width, height } => {
                            Events::WindowEvents {
                                id,
                                event: WindowEvents::FramebufferChanged { width, height },
                            }
                        }
                        MainWindowEvents::Moved { x, y } => Events::WindowEvents {
                            id,
                            event: WindowEvents::Moved { x, y },
                        },
                        MainWindowEvents::SetFocus => Events::WindowEvents {
                            id,
                            event: WindowEvents::SetFocus,
                        },
                        MainWindowEvents::LostFocus => Events::WindowEvents {
                            id,
                            event: WindowEvents::LostFocus,
                        },
                        MainWindowEvents::RedrawRequested => Events::WindowEvents {
                            id,
                            event: WindowEvents::RedrawRequested,
                        }
                    },
                    MainEvents::MainKeyboardEvent { id, event } => match event {
                        MainKeyboardEvents::Key {
                            up,
                            is_changed,
                            keycode,
                        } => {
                            if keycode == Key::ALT {
                                println!("Alt up: {}, changed: {}, keycode: {}, is_down: {}, is_changed: {}, is_released: {}", up, is_changed, keycode, self.keyboard.is_down(keycode), self.keyboard.is_changed(keycode), self.keyboard.is_released(keycode));

                                Events::KeyboardEvents {
                                    id,
                                    event: KeyboardEvents::Key {
                                        keycode,
                                        action: Action::Release,
                                    },
                                }
                            } else
                            if up {
                                self.keyboard.set_is_down(keycode, false);
                                self.keyboard.set_is_changed(keycode, true);
                                self.keyboard.set_is_released(keycode, true);

                                Events::KeyboardEvents {
                                    id,
                                    event: KeyboardEvents::Key {
                                        keycode,
                                        action: Action::Release,
                                    },
                                }
                            } else {
                                self.keyboard.set_is_down(keycode, true);
                                self.keyboard.set_is_changed(keycode, is_changed);

                                if !self.keyboard.is_changed(keycode) {
                                    Events::KeyboardEvents {
                                        id,
                                        event: KeyboardEvents::Key {
                                            keycode,
                                            action: Action::Down,
                                        },
                                    }
                                } else if self.keyboard.is_released(keycode) {
                                    self.keyboard.set_is_released(keycode, false);
                                    Events::KeyboardEvents {
                                        id,
                                        event: KeyboardEvents::Key {
                                            keycode,
                                            action: Action::Press,
                                        },
                                    }
                                } else {
                                    Events::None
                                }
                            }
                        }

                        MainKeyboardEvents::Char { keycode } => {
                            self.keyboard.set_is_char(keycode, true);
                            Events::KeyboardEvents {
                                id,
                                event: KeyboardEvents::Char { keycode },
                            }
                        }
                    },
                    MainEvents::MainMouseEvent { id, event } => match event {
                        MainMouseEvents::Scroll { y_offset } => Events::MouseEvents {
                            id,
                            event: MouseEvents::Scroll { y_offset },
                        },
                        MainMouseEvents::LButton { up, pos } => {
                            if up {
                                self.mouse.set_l_button_down(false);
                                self.mouse.set_l_button_released(true);
                                self.mouse.set_l_button_changed(true);

                                Events::MouseEvents {
                                    id,
                                    event: MouseEvents::LButton {
                                        action: Action::Release,
                                        pos,
                                    },
                                }
                            } else {
                                self.mouse.set_l_button_down(true);
                                self.mouse.set_l_button_released(false);
                                self.mouse.set_l_button_changed(true);

                                Events::MouseEvents {
                                    id,
                                    event: MouseEvents::LButton {
                                        action: Action::Press,
                                        pos,
                                    },
                                }
                            }
                        }
                        MainMouseEvents::RButton { up, pos } => {
                            if up {
                                self.mouse.set_r_button_down(false);
                                self.mouse.set_r_button_released(true);
                                self.mouse.set_r_button_changed(true);

                                Events::MouseEvents {
                                    id,
                                    event: MouseEvents::LButton {
                                        action: Action::Release,
                                        pos,
                                    },
                                }
                            } else {
                                self.mouse.set_r_button_down(true);
                                self.mouse.set_r_button_released(false);
                                self.mouse.set_r_button_changed(true);

                                Events::MouseEvents {
                                    id,
                                    event: MouseEvents::LButton {
                                        action: Action::Press,
                                        pos,
                                    },
                                }
                            }
                        }
                        MainMouseEvents::MButton { up, pos } => {
                            if up {
                                self.mouse.set_m_button_down(false);
                                self.mouse.set_m_button_released(true);
                                self.mouse.set_m_button_changed(true);

                                Events::MouseEvents {
                                    id,
                                    event: MouseEvents::LButton {
                                        action: Action::Release,
                                        pos,
                                    },
                                }
                            } else {
                                self.mouse.set_m_button_down(true);
                                self.mouse.set_m_button_released(false);
                                self.mouse.set_m_button_changed(true);

                                Events::MouseEvents {
                                    id,
                                    event: MouseEvents::LButton {
                                        action: Action::Press,
                                        pos,
                                    },
                                }
                            }
                        }
                        MainMouseEvents::XButton { up, wparam, pos } => {
                            if up {
                                if HIWORD(wparam) & XBUTTON1 > 0 {
                                    self.mouse.set_x1_button_down(false);
                                    self.mouse.set_x1_button_released(true);
                                    self.mouse.set_x1_button_changed(true);

                                    Events::MouseEvents {
                                        id,
                                        event: MouseEvents::X1Button {
                                            action: Action::Release,
                                            pos,
                                        },
                                    }
                                } else if HIWORD(wparam) & XBUTTON2 > 0 {
                                    self.mouse.set_x2_button_down(false);
                                    self.mouse.set_x2_button_released(true);
                                    self.mouse.set_x2_button_changed(true);

                                    Events::MouseEvents {
                                        id,
                                        event: MouseEvents::X2Button {
                                            action: Action::Release,
                                            pos,
                                        },
                                    }
                                } else {
                                    Events::None
                                }
                            } else {
                                if LOWORD(wparam as u32) as usize & MK_XBUTTON1 > 0 {
                                    self.mouse.set_x1_button_down(true);
                                    self.mouse.set_x1_button_released(false);
                                    self.mouse.set_x1_button_changed(true);

                                    Events::MouseEvents {
                                        id,
                                        event: MouseEvents::X1Button {
                                            action: Action::Press,
                                            pos,
                                        },
                                    }
                                } else if LOWORD(wparam as u32) as usize & MK_XBUTTON2 > 0 {
                                    self.mouse.set_x2_button_down(true);
                                    self.mouse.set_x2_button_released(false);
                                    self.mouse.set_x2_button_changed(true);

                                    Events::MouseEvents {
                                        id,
                                        event: MouseEvents::X2Button {
                                            action: Action::Press,
                                            pos,
                                        },
                                    }
                                } else {
                                    Events::None
                                }
                            }
                        }
                        MainMouseEvents::MouseMove { x, y } => {
                            self.mouse.update_pos(x, y);
                            Events::MouseEvents {
                                id,
                                event: MouseEvents::MouseMove {
                                    x: self.mouse.x(),
                                    y: self.mouse.y(),
                                    last_x: self.mouse.last_x(),
                                    last_y: self.mouse.last_y(),
                                    dx: self.mouse.x_offset(),
                                    dy: self.mouse.y_offset(),
                                },
                            }
                        }
                    },
                };

                func(events, &mut control_flow, self);

                match control_flow {
                    ControlFlow::Continue => {}
                    ControlFlow::Exit => {
                        self.close = true;
                        break 'user_events_loop;
                    }
                    ControlFlow::ExitWithCode(exit_code) => {
                        panic!("Exit code with {}", exit_code);
                    }
                }
            }
            self.keyboard.clear();
            self.mouse.clear_keystates();

            func(Events::default(), &mut control_flow, self);
            match control_flow {
                ControlFlow::Continue => {}
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

    unsafe fn wndproc(
        msger: &Messenger,
        hwnd: HWND,
        msg: UINT,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        match msg {
            WM_DESTROY => {
                // println!("{}", Window::get_hwnd_class_name(hwnd));

                msger.send(MainEvents::MainWindowEvent {
                    id: hwnd as usize,
                    event: MainWindowEvents::Close,
                });
                PostQuitMessage(0);
            }

            WM_MOUSEMOVE => {
                let x = MAKEPOINTS(lparam as u32).x;
                let y = MAKEPOINTS(lparam as u32).y;

                msger.send(MainEvents::MainMouseEvent {
                    id: hwnd as usize,
                    event: MainMouseEvents::MouseMove { x, y },
                });
            }

            WM_MOUSEWHEEL => {
                let delta = GET_WHEEL_DELTA_WPARAM(wparam);
                msger.send(MainEvents::MainMouseEvent {
                    id: hwnd as usize,
                    event: MainMouseEvents::Scroll {
                        y_offset: delta / WHEEL_DELTA,
                    },
                });
            }

            WM_CHAR => {
                msger.send(MainEvents::MainKeyboardEvent {
                    id: hwnd as usize,
                    event: MainKeyboardEvents::Char { keycode: wparam },
                });
            }

            WM_KEYDOWN | WM_SYSKEYDOWN => {
                msger.send(MainEvents::MainKeyboardEvent {
                    id: hwnd as usize,
                    event: MainKeyboardEvents::Key {
                        up: false,
                        keycode: wparam,
                        is_changed: (lparam & (1 << 30)) == 0,
                    },
                });
            }

            WM_KEYUP | WM_SYSKEYUP => {
                msger.send(MainEvents::MainKeyboardEvent {
                    id: hwnd as usize,
                    event: MainKeyboardEvents::Key {
                        up: true,
                        keycode: wparam,
                        is_changed: (lparam & (1 << 30)) == 0,
                    },
                });
            }

            WM_SIZE => {
                let width = LOWORD(lparam as u32) as i32;
                let height = HIWORD(lparam as u32) as i32;
                if wparam == SIZE_MAXIMIZED {
                    msger.send(MainEvents::MainWindowEvent {
                        id: hwnd as usize,
                        event: MainWindowEvents::Maximized { width, height },
                    });
                } else if wparam == SIZE_MINIMIZED {
                    msger.send(MainEvents::MainWindowEvent {
                        id: hwnd as usize,
                        event: MainWindowEvents::Minimized { width, height },
                    });
                } else {
                    msger.send(MainEvents::MainWindowEvent {
                        id: hwnd as usize,
                        event: MainWindowEvents::FramebufferChanged { width, height },
                    });
                }
            }

            WM_MOVE => {
                let x = LOWORD(lparam as u32) as i32;
                let y = HIWORD(lparam as u32) as i32;
                msger.send(MainEvents::MainWindowEvent {
                    id: hwnd as usize,
                    event: MainWindowEvents::Moved { x, y },
                });
            }

            WM_LBUTTONDOWN => {
                let x = LOWORD(lparam as u32) as i32;
                let y = HIWORD(lparam as u32) as i32;

                msger.send(MainEvents::MainMouseEvent {
                    id: hwnd as usize,
                    event: MainMouseEvents::LButton {
                        up: false,
                        pos: Point::new(x, y),
                    },
                });
            }

            WM_LBUTTONUP => {
                let x = LOWORD(lparam as u32) as i32;
                let y = HIWORD(lparam as u32) as i32;

                msger.send(MainEvents::MainMouseEvent {
                    id: hwnd as usize,
                    event: MainMouseEvents::LButton {
                        up: true,
                        pos: Point::new(x, y),
                    },
                });
            }

            WM_RBUTTONDOWN => {
                let x = LOWORD(lparam as u32) as i32;
                let y = HIWORD(lparam as u32) as i32;

                msger.send(MainEvents::MainMouseEvent {
                    id: hwnd as usize,
                    event: MainMouseEvents::RButton {
                        up: false,
                        pos: Point::new(x, y),
                    },
                });
            }

            WM_RBUTTONUP => {
                let x = LOWORD(lparam as u32) as i32;
                let y = HIWORD(lparam as u32) as i32;

                msger.send(MainEvents::MainMouseEvent {
                    id: hwnd as usize,
                    event: MainMouseEvents::RButton {
                        up: true,
                        pos: Point::new(x, y),
                    },
                });
            }

            WM_MBUTTONDOWN => {
                let x = LOWORD(lparam as u32) as i32;
                let y = HIWORD(lparam as u32) as i32;

                msger.send(MainEvents::MainMouseEvent {
                    id: hwnd as usize,
                    event: MainMouseEvents::MButton {
                        up: false,
                        pos: Point::new(x, y),
                    },
                });
            }

            WM_MBUTTONUP => {
                let x = LOWORD(lparam as u32) as i32;
                let y = HIWORD(lparam as u32) as i32;

                msger.send(MainEvents::MainMouseEvent {
                    id: hwnd as usize,
                    event: MainMouseEvents::MButton {
                        up: true,
                        pos: Point::new(x, y),
                    },
                });
            }

            WM_XBUTTONDOWN => {
                let x = LOWORD(lparam as u32) as i32;
                let y = HIWORD(lparam as u32) as i32;

                msger.send(MainEvents::MainMouseEvent {
                    id: hwnd as usize,
                    event: MainMouseEvents::XButton {
                        up: false,
                        pos: Point::new(x, y),
                        wparam: wparam as u32,
                    },
                });
            }

            WM_XBUTTONUP => {
                let x = LOWORD(lparam as u32) as i32;
                let y = HIWORD(lparam as u32) as i32;

                msger.send(MainEvents::MainMouseEvent {
                    id: hwnd as usize,
                    event: MainMouseEvents::XButton {
                        up: true,
                        pos: Point::new(x, y),
                        wparam: wparam as u32,
                    },
                });
            }

            WM_SETFOCUS => {
                msger.send(MainEvents::MainWindowEvent {
                    id: hwnd as usize,
                    event: MainWindowEvents::SetFocus,
                });
            }

            WM_KILLFOCUS => {
                msger.send(MainEvents::MainWindowEvent {
                    id: hwnd as usize,
                    event: MainWindowEvents::LostFocus,
                });
            }

            WM_PAINT => {
                msger.send(MainEvents::MainWindowEvent {
                    id: hwnd as usize,
                    event: MainWindowEvents::RedrawRequested,
                });
            }

            _ => {}
        }

        return DefWindowProcW(hwnd, msg, wparam, lparam);
    }

    unsafe extern "system" fn process_messages(
        hwnd: HWND,
        msg: UINT,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        let msger = (GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *const Messenger)
            .as_ref()
            .unwrap();

        return Self::wndproc(msger, hwnd, msg, wparam, lparam);
    }

    unsafe extern "system" fn setup(
        hwnd: HWND,
        msg: UINT,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        if msg == WM_CREATE {
            let create_struct = lparam as *mut CREATESTRUCTW;
            let msger_ptr = create_struct.as_ref().unwrap().lpCreateParams as *const Messenger;
            let msger = msger_ptr.as_ref().unwrap();
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, msger_ptr as isize);
            SetWindowLongPtrW(hwnd, GWLP_WNDPROC, Self::process_messages as isize);
            msger.send(MainEvents::MainWindowEvent {
                id: hwnd as usize,
                event: MainWindowEvents::Create,
            });
            return Self::wndproc(msger, hwnd, msg, wparam, lparam);
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

            _ => {
                panic!("There is no virtual mouse button code like {button}");
            }
        }
    }

    /// Retrieves the current frame and delta time
    ///
    /// # Example
    ///
    /// ```ignore
    /// println!("Time is {}", manager.time().0);
    /// ```
    pub fn time(&mut self) -> (f32, f32) {
        return (self.timer.current_frame, self.timer.dt());
    }

    /// If all the windows are closed returns true else return false
    ///
    /// # Example
    ///
    /// ```ignore
    /// println!("Has all the windows been closed: {}", manager.all_closed());
    /// ```
    pub fn all_closed(&self) -> bool {
        if self.windows.len() == 0 {
            true
        } else {
            false
        }
    }
}
