use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

use log::info;

use crate::init_log;

#[derive(Clone, Copy)]
pub struct Server;

impl Server {
    fn new() -> Self {
        Self {}
    }
    pub fn init(add: String) {
        let server = Arc::new(Server::new());
        let listener = TcpListener::bind(&add).unwrap();
        println!("Server listening on {}", add);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let server = Arc::clone(&server);
                    std::thread::spawn(move || {
                        server.handle_client(stream);
                    });
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }
    fn handle_client(&self, mut stream: TcpStream) {
        // 读取消息
        let message = match self.read_message(&mut stream) {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("Failed to read message: {}", e);
                return;
            }
        };
        
        info!("ip地址为{}",stream.peer_addr().unwrap());
        info!("Received from client: {}", message);

        // 保存消息，这里我们仅将其打印出来
        // 在实际应用中，你可能会将其存储在文件、数据库或其他存储系统中
        let saved_message = message;

        // 向客户端发送确认消息
        let response = format!("Message received:\"{}\" ", saved_message);
        if let Err(e) = stream.write_all(response.as_bytes()) {
            eprintln!("Failed to send confirmation: {}", e);
        }
    }

    fn read_message(&self, stream: &mut TcpStream) -> io::Result<String> {
        let mut message = String::new();
        let mut buffer = [0; 1024];
        while let Ok(bytes_read) = stream.read(&mut buffer) {
            if bytes_read == 0 {
                break; // 连接关闭
            }
            message.push_str(&String::from_utf8_lossy(&buffer[..bytes_read]));
            if message.ends_with('\n') {
                // info!()
                break; // 消息结束
            }
        }
        Ok(message)
    }
}

#[test]
fn test_server() {
    init_log();
    info!("初始化p2p服务端节点");
    Server::init("192.168.21.233:12345".to_owned());
}
