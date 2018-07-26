#![feature(box_syntax)]

#[cfg_attr(
    all(not(target_arch = "wasm32"), feature = "json"),
    macro_use
)]
extern crate log;

#[cfg(not(target_arch = "wasm32"))]
extern crate websocket;

#[cfg(feature = "protobuf")]
extern crate protobuf;

#[cfg(feature = "json")]
#[macro_use]
extern crate serde_derive;
#[cfg(feature = "json")]
extern crate serde;
#[cfg(feature = "json")]
extern crate serde_json;

pub mod chan;
pub mod error;
pub mod event;
pub mod message;
#[cfg(feature = "protobuf")]
pub mod protos;
#[cfg(all(not(target_arch = "wasm32"), feature = "json"))]
pub mod socket;

pub use chan::Channel;
pub use error::Error;
pub use event::Event;
pub use message::Message;
#[cfg(all(not(target_arch = "wasm32"), feature = "json"))]
pub use socket::Phoenix;
