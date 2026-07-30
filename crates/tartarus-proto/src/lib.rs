//! Tartarus event protocol.
//!
//! The contract between the guest agent and the host orchestrator. This is the
//! ONLY thing both sides agree on. Everything else is an implementation detail
//! on one side or the other.
//!
//! ============================================================================
//! ASSIGNMENT 1 — write this crate. No dependencies. No I/O. Types only.
//! ============================================================================
//!
//! Define three events: a Process executing, a file being accessed, and a
//! network connection being made. Then define one type that represents
//! "any event", so the orchestrator can hold a stream of them.
//!
//! Constraints (these ARE the exercise — do not route around them):
//!
//!   1. No `String` fields until you can tell me why `&str` does not work here.
//!   2. Every event carries a timestamp and the PID that caused it. Decide
//!      whether that lives on each event or somewhere else, and defend it.
//!   3. An IPv4 and an IPv6 connection must not be representable as the same
//!      thing by accident. Make the type system stop it.
//!   4. `cargo clippy` clean. Warnings are errors here.
//!
//!
//!
use std::net::SocketAddr;

#[derive(Debug)]
pub struct Process {
    pub command_line: String,
    pub executable_path: String,
    pub parent_pid: Option<u32>,
}

#[derive(Debug)]
pub enum AccessType {
    Read,
    Write,
    Delete,
}

#[derive(Debug)]
pub struct FileAccess {
    pub file_path: String,
    pub access_type: AccessType,
}

#[derive(Debug)]
pub enum Protocol {
    Tcp,
    Udp,
}

#[derive(Debug)]
pub enum Direction {
    Incoming,
    Outgoing,
}

#[derive(Debug)]
pub struct NetworkConnection {
    pub protocol: Protocol,
    pub direction: Direction,
    pub local: SocketAddr,
    pub remote: SocketAddr,
}

#[derive(Debug)]
pub enum RawEvent {
    Process(Process),
    FileAccess(FileAccess),
    NetworkConnection(NetworkConnection),
}

#[derive(Debug)]
pub struct Event {
    pub pid: u32,
    /// nano seceends. 
    /// monotonic, later synced with the host
    pub timestamp: u64,
    pub raw_event: RawEvent,
}
