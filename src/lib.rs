//! # Cursive
//!
//! [Cursive] is a [TUI] library - it lets you easily build rich interfaces
//! for use in a terminal.
//!
//! [Cursive]: https://github.com/gyscos/Cursive
//! [TUI]: https://en.wikipedia.org/wiki/Text-based_user_interface
//!
//! ## Getting started
//!
//! * Every application should start with a [`Cursive`](struct.Cursive.html)
//!   object. It is the main entry-point to the library.
//! * A declarative phase then describes the structure of the UI by adding
//!   views and configuring their behaviours.
//! * Finally, the event loop is started by calling
//!   [`Cursive::run(&mut self)`](struct.Cursive.html#method.run).
//!
//! ## Views
//!
//! Views are the main components of a cursive interface.
//! The [`views`](./views/index.html) module contains many views to use in your
//! application; if you don't find what you need, you may also implement the
//! [`View`](view/trait.View.html) trait and build your own.
//!
//! ## Callbacks
//!
//! Cursive is a *reactive* UI: it *reacts* to events generated by user input.
//!
//! During the declarative phase, callbacks are set to trigger on specific
//! events. These functions usually take a `&mut Cursive` argument, allowing
//! them to modify the view tree at will.
//!
//! ## Examples
//!
//! ```rust
//! extern crate cursive;
//!
//! use cursive::Cursive;
//! use cursive::views::TextView;
//!
//! fn main() {
//!     let mut siv = Cursive::dummy();
//!
//!     siv.add_layer(TextView::new("Hello World!\nPress q to quit."));
//!
//!     siv.add_global_callback('q', |s| s.quit());
//!
//!     siv.run();
//! }
//! ```
//!
//! ## Debugging
//!
//! The `Cursive` root initializes the terminal on creation, and do cleanups
//! on drop. While it is alive, printing to the terminal will not work
//! as expected, making debugging a bit harder.
//!
//! One solution is to redirect stderr to a file when running the application,
//! and log to it instead of stdout.
//!
//! Or you can use gdb as usual.
#![deny(missing_docs)]

#[macro_use]
extern crate enum_map;
#[macro_use]
extern crate enumset;
#[macro_use]
extern crate log;
#[macro_use]
extern crate chan;

#[cfg(any(feature = "ncurses", feature = "pancurses"))]
#[macro_use]
extern crate maplit;

// We use chan_signal to detect SIGWINCH.
// It's not how windows work, so no need to use that.
#[cfg(unix)]
extern crate chan_signal;

extern crate libc;
extern crate num;
extern crate owning_ref;
extern crate toml;
extern crate unicode_segmentation;
extern crate unicode_width;
extern crate xi_unicode;

macro_rules! new_default(
    ($c:ty) => {
        impl Default for $c {
            fn default() -> Self {
                Self::new()
            }
        }
    }
);

pub mod traits;

pub mod event;
#[macro_use]
pub mod view;

pub mod align;
pub mod direction;
pub mod menu;
pub mod rect;
pub mod theme;
pub mod utils;
pub mod vec;
pub mod views;

// This probably doesn't need to be public?
mod cursive;
mod printer;
mod with;
mod xy;

mod div;
mod utf8;

#[doc(hidden)]
pub mod backend;

pub use cursive::{CbFunc, Cursive, ScreenId};
pub use printer::Printer;
pub use vec::Vec2;
pub use with::With;
pub use xy::XY;
