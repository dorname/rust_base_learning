use server::Server;

pub mod client;
pub mod server;
//p2p节点之间的建立通信
//会话密钥
//通信安全
struct Node {
    id: String,
}

impl Node {
    fn new(id: String, address: String) -> Self {
        Server::init(address);
        Self { id: id }
    }
}
