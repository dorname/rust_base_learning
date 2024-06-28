
mod node;
use log::*;
use log4rs;

fn initLog(){
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
}