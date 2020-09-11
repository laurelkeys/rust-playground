use std::fs;
use std::io::prelude::{Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream).unwrap();
    }
}

// @Fixme: better handle multiple error types, see:
// https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types.html
fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 1024];

    let _n = stream.read(&mut buffer)?;

    // HTTP request format:
    //  |
    //  |   Method Request-URI HTTP-Version CRLF
    //  |   headers CRLF
    //  |   message-body
    //
    // HTTP response format:
    //  |
    //  |   HTTP-Version Status-Code Reason-Phrase CRLF
    //  |   headers CRLF
    //  |   message-body
    //

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename)?;

    let response = format!("{}{}", status_line, contents);

    let _n = stream.write(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}
