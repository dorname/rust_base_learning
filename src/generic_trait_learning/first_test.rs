use std::fmt;
struct worker{
    name:String,
    workerId:i32,
    
}
impl fmt::Debug for worker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Worker")
            .field("name", &self.name)
            .field("workerId", &self.workerId)
            .finish()
    }
}
impl  worker {
    fn self_produce(&self){
        println!("{}",self.name);
    }
    
}
trait behavior{
    type Tool;
    const YEAR:i32 = 2023;
    fn start(&self) {
        println!("start working!!!");
    }
    fn work(&self){
        println!("coding");
    }
    fn end(&self){
        println!("end working!!!");
    }
    fn talking_shit<T:fmt::Debug>(&self,name: &T,msg: &T){
        println!("{:?}is ^*** {:?}",name,msg);
    }
    fn init_tool(&self,x: Self::Tool);
}

impl behavior for worker{
    type Tool = String;
    fn init_tool(&self,x: Self::Tool){
        println!("{:?},{:?}",Self::YEAR,x);
    }
}
#[test]
fn test(){
    let w = worker{
        name:String::from("mengzhongshadouyou"),
        workerId:123
    };
    w.init_tool(String::from("getting keybroad"));
    w.start();
    w.work();
    w.end();
    w.self_produce();
    w.talking_shit(&String::from("boss"), &String::from("idiot"));
    println!("{:?}",w);
}