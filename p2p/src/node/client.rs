
use std::net::TcpStream;
use std::io::{self, Read, Write};
use std::str;

#[test]
fn client() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:12345")?;
    println!("Successfully connected to server in port 12345");

    let msg = "Hello!\n";
    stream.write_all(msg.as_bytes())?;
    println!("Sent Hello, awaiting reply...");

    let mut data = [0 as u8; 50]; // Use a buffer larger than the message to receive the server's response.
    let bytes_read = stream.read(&mut data)?;
    let response = str::from_utf8(&data[..bytes_read]).expect("Received non-UTF8 message");

    println!("Reply: {}", response);
    Ok(())
}

