
use std::net::TcpStream;
use std::io::{self, Error, Read, Write};
use std::str;

struct Client;
impl Client {
    fn new()->Self{
        Self{}
    }
    fn connect(addr:String,msg:String)-> io::Result<()>{
        let client = Client::new();
        let mut stream = TcpStream::connect(&addr)?;
        println!("Successfully connected to server {}",addr);
        client.send(msg, &mut stream);
        let response = client.get_response(&mut stream)?;
        println!("Reply:{}",response);
        Ok(())
    }
    
    fn send(&self,msg:String,stream:&mut TcpStream)->io::Result<()>{
        println!("Sending {}",msg);
        stream.write_all(msg.as_bytes())?;
        Ok(())
    }
    
    fn get_response(&self,stream:&mut TcpStream)->Result<String, Error>{
        let mut data = [0 as u8; 50]; // Use a buffer larger than the message to receive the server's response.
        let bytes_read = stream.read(&mut data)?;
        let response = str::from_utf8(&data[..bytes_read]).expect("Received non-UTF8 message");
        Ok(response.to_owned())
    }    
}

#[test]
fn test_client() {
    Client::connect("127.0.0.1:12345".to_owned(),"Hi\n".to_owned());
}

