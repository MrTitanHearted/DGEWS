//! # DGEWS 
//! 
//! DGEWS is a multithreaded toy windowing system for learning win32 api in rust lang.

//! # Example
//! 
//! ```rust,ignore
//! extern crate dgews;
//! use dgews::prelude::*; // prelude module contains everything
//! 
//! fn main() {
//!     let mut manager = Manager::new(WindowBuilder::default()
//!         .with_title("DGEWS Window")
//!         .with_dimensions(800, 640)
//!         .with_theme(Theme::Dark));
//!     
//!     manager.run(|events, control_flow, manager| {
//!         match events {
//!             Events::WindowEvent { id, event } => match event {
//!                 WindowEvents::Create => println!("[INFO]: a new window with id: {} has been created", manager.window().get_id()),
//! 
//!                 WindowEvents::Close => {
//!                     println!("[INFO]: a window with id: {} has been closed", manager.window().get_id());
//!                     *control_flow => ControlFlow::Exit; // to exit with panicing, use ControlFlow::ExitWithCode(<your number>) instead.
//!                 },
//! 
//!                 _=> {}
//!             },
//! 
//!             Events::MouseEvent { id, event } => match event {
//!                 MouseEvents::MouseMove { x, y, last_x, last_y, dx, dy } => {
//!                     println!("[INFO]: mouse moved in the window with id {}: x={}, y={}, last_x={}, last_y={} dx={} dy={};", manager.window().get_id(), x, y, last_x, last_y, dx, dy);
//!                 },
//!                 
//!                 _=> {}
//!             }
//! 
//!             _=> *control_flow = ControlFlow::Continue,
//!         }
//! 
//!         if manager.get_key(Key::ESCAPE) == Action::Release {
//!             println!("[INFO]: program is exiting");
//!             manager.close(); // or *control_flow = ControlFlow::Exit;
//!         }
//!     });
//! }
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