#[tokio::test]
async fn test()-> Result<(), Box<dyn std::error::Error>>  {
    use std::net::SocketAddr;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    //
    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(&addr).await?;

    println!("Listening on: http://{}", addr);

    loop {
        let (mut stream, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            stream.read(&mut buffer).await.unwrap();

            let contents = "<h1>Hello, world!</h1>";
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                contents.len(),
                contents
            );

            stream.write_all(response.as_bytes()).await.unwrap();
            stream.flush().await.unwrap();
        });
    }
}
