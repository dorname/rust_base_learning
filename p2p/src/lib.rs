
mod node;
use log4rs;

fn init_log(){
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
}