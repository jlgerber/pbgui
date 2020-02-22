//! The messaging sub-module is responsible for implementing a unidirectional dataflow
//! model for the ui. Pbgui draws inspiration from predecessors like the Elm Architecture
//! to model state changes in terms of a request / response cycle, using mpsc channels to communicate
//! between the gui and a request processessor.
//!
//! Pbgui defines both incoming and outgoing messages to serve requests and responses.
pub mod incoming;
pub use incoming::{IMsg, IPackagesTree, IVpinDialog, ToIMsg};
pub mod outgoing;
pub use outgoing::{OMsg, OPackagesTree, OVpinDialog, ToOMsg};
pub mod event;
pub use event::{Event, ToEvent, VpinDialog};
pub mod event_handler;
pub use event_handler::new_event_handler;
pub mod client_proxy;
pub mod init;
pub mod thread;
pub use crossbeam_channel::{unbounded as channel, Receiver, Sender};

pub mod prelude {
    pub use super::event::ToEvent;
    pub use super::incoming::ToIMsg;
    pub use super::outgoing::ToOMsg;
    pub use qt_thread_conductor::traits::*;
}
