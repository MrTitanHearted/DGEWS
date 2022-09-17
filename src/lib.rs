pub(crate) extern crate winapi;
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
    pub(crate) extern crate winapi;
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