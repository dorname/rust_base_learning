use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    // 读取消息
    let message = match read_message(&mut stream) {
        Ok(msg) => msg,
        Err(e) => {
            eprintln!("Failed to read message: {}", e);
            return;
        }
    };

    println!("Received from client: {}", message);

    // 保存消息，这里我们仅将其打印出来
    // 在实际应用中，你可能会将其存储在文件、数据库或其他存储系统中
    let saved_message = message;

    // 向客户端发送确认消息
    let response = format!("Message received  \"{}\"", "ok");
    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Failed to send confirmation: {}", e);
    }
}

fn read_message(stream: &mut TcpStream) -> io::Result<String> {
    let mut message = String::new();
    let mut buffer = [0; 1024];
    while let Ok(bytes_read) = stream.read(&mut buffer) {
        if bytes_read == 0 {
            break; // 连接关闭
        }
        message.push_str(&String::from_utf8_lossy(&buffer[..bytes_read]));
        if message.ends_with('\n') {
            break; // 消息结束
        }
    }
    Ok(message)
}

#[test]
fn server() {
    let listener = TcpListener::bind("127.0.0.1:12345").unwrap();
    println!("Server listening on port 12345");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
