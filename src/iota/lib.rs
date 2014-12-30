#![feature(unboxed_closures)]
#![feature(unsafe_destructor)]
#![feature(phase)]
#![feature(slicing_syntax)]

#[phase(plugin,link)] extern crate log;
#[phase(plugin,link)] extern crate racc;
extern crate rustbox;
extern crate gapbuffer;

pub use editor::Editor;
pub use input::Input;
pub use frontends::RustboxFrontend;

mod input;
mod utils;
mod buffer;
mod editor;
mod keyboard;
mod keymap;
mod view;
mod uibuf;
mod tlog;
mod frontends;
mod query;

#[deriving(Copy)]
pub enum Response {
    Continue,
    Quit,
}
