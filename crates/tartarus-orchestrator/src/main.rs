use std::{
    io::{BufRead, BufReader},
    os::unix::net::{UnixListener, UnixStream},
    path::Path,
};

use tartarus_proto::Event;

fn main() {
    let socket = Path::new("/tmp/tartarus.sock");

    let listener = UnixListener::bind(socket).unwrap();

    println!("Connected on socket: {:?}", socket);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(err) = handle_client(stream) {
                    eprintln!("client connection failed: {err}");
                }
            }
            Err(err) => {
                eprintln!("accept failed: {err}");
                break;
            }
        }
    }

    let _ = std::fs::remove_file(socket);
}

fn handle_client(stream: UnixStream) -> std::io::Result<()> {
    let mut stream = BufReader::new(stream);

    let mut buf: Vec<u8> = Vec::new();
    loop {
        buf.clear();
        let n = stream.read_until(b'\n', &mut buf)?;
        if n == 0 {
            break;
        }
        match serde_json::from_slice::<Event>(&buf) {
            Ok(event) => println!("{event:#?}"),
            Err(err) => eprintln!("malformed event dropped: {err}"),
        }
    }
    Ok(())
}
