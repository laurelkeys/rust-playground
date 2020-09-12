use ch20_hello as hello;
use hello::ThreadPool;
use std::fs;
use std::io::prelude::{Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    // Shutdown after two requests.
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream).unwrap();
        });
    }

    println!("Shutting down.");
}

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
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
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
