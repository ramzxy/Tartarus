use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Process {
    pub command_line: String,
    pub executable_path: String,
    pub parent_pid: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccessType {
    Read,
    Write,
    Delete,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct FileAccess {
    pub file_path: String,
    pub access_type: AccessType,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Protocol {
    Tcp,
    Udp,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    Incoming,
    Outgoing,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct NetworkConnection {
    pub protocol: Protocol,
    pub direction: Direction,
    pub local: SocketAddr,
    pub remote: SocketAddr,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RawEvent {
    Process(Process),
    FileAccess(FileAccess),
    NetworkConnection(NetworkConnection),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Event {
    pub pid: u32,
    /// Nanoseconds since guest boot, monotonic. Ordering only, not a date.
    pub timestamp: u64,
    pub raw_event: RawEvent,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_round_trip() {
        let original = Event {
            pid: 123,
            timestamp: 456,
            raw_event: RawEvent::Process(Process {
                command_line: "ls -la".to_string(),
                executable_path: "/bin/ls".to_string(),
                parent_pid: Some(1),
            }),
        };
        let json = serde_json::to_string(&original).unwrap();
        let back = serde_json::from_str::<Event>(&json).unwrap();
        assert_eq!(original, back);
    }
}
