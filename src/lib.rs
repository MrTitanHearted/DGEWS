//! # DGEWS 
//! 
//! DGEWS is a multithreaded toy windowing system for learning win32 api in rust lang.

//! # Example
//! 
//! ```rust,ignore
//! let mut manager = Manager::new(WindowBuilder::default()
//!         .with_title("Hello, World!")
//!         .with_dimensions(1200, 1000)
//!         .with_icon("myIcon.ico"))
//!     .add_window("AnotherWindow", WindowBuilder::default()
//!         .with_title("Another Window for Debugging")
//!         .with_pos(600, 600)
//!         .with_dimensions(640, 800)
//!         .with_theme(Theme::Dark));
//! 
//! manager.run(|events, control_flow, manager| {
//!     match events {
//!         Events::WindowEvent { id, event } => match event {
//!             WindowEvents::Close => if id == manager.get_window("AnotherWindow").get_id() {
//!                 *control_flow = ControlFlow::Exit;
//!             }
//!             _=> {}
//!         }
//!         Events::KeyboardEvent { id: _, event } => match event {
//!             KeyboardEvents::Key { keycode, action } => if keycode == Key::H && action == Action::Release {
//!                 println!("A key is released!");    
//!             }
//!             _=> {}
//!         }
//!         Events::MouseEvent { id, event } => match event {
//!             MouseEvents::LButton { action, pos } => if id == manager.window().get_id() && action == Action::Press {
//!                 println!("Left mouse button is pressed on the window with id: {id} in the position of (x: {pos.x}, y: {pos.y})");
//!             }
//!             _=> {}
//!         }
//!         _=> {},
//!     }
//! 
//!     if manager.get_key(Key::ESCAPE) == Action::Release {
//!         manager.close();
//!     }
//! });
//! ```

pub(crate) mod keyboard;
pub(crate) mod keystates;
pub(crate) mod mouse;

pub mod common;
pub mod controlflow;
pub mod events;
pub mod keycodes;
pub mod manager;
pub mod timer;
pub mod window;
pub mod windowbuilder;

pub mod prelude {
    pub(crate) use super::keyboard::*;
    pub(crate) use super::keystates::*;
    pub(crate) use super::mouse::*;
    
    pub use super::common::*;
    pub use super::controlflow::*;
    pub use super::events::*;
    pub use super::keycodes::*;
    pub use super::manager::*;
    pub use super::timer::*;
    pub use super::window::*;
    pub use super::windowbuilder::*;
}