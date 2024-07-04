use log::*;
use byteorder::{ByteOrder,BigEndian};
use log4rs::append::rolling_file::policy::compound::trigger::size;

use crate::const_var::*;
#[derive(Debug)]
struct Evm {
    //以太坊虚拟机字节码
    code: Vec<u8>,
    //程序计数器
    pc: usize,
    //堆栈
    //每个元素长度为256位（32字节），最大深度为1024元素，但是每个操作只能操作堆栈顶的16个元素
    stack: Vec<u32>
}

/// 为虚拟机实现其特征行为和方法
/// 1）初始化方法，
///    字节码为入参
///    计数器初始值为0
///    堆栈初始化为空
impl Evm {
    ///初始化虚拟机
    fn new(code:Vec<u8>) -> Self{
        init_log();
        Evm { code: code, pc: 0, stack: Vec::<u32>::new() }
    }   

    /// 获取当前待执行的指令
    fn get_current_instruction(&mut self) -> u8 {
      
        let &op:&u8 = self.code.get(self.pc).unwrap();
        info!("当前执行的指令为{}",op);
        self.pc += 1;
        info!("程序计数器累加1结果为:{}",self.pc);
        return op.clone();
    }

    fn run(&mut self){
        while self.pc  < self.code.len() {
            let op:u8 = self.get_current_instruction();
            
            match op {
                op if (PUSH1 <= op && op <= PUSH32) => {
                    let size = (op - PUSH1 + 1) as usize;
                    self.push(size);
                }
                PUSH0 => {
                    self.stack.push(0);
                    // self.pc += size; // 此行应当被删除或者注释掉，因为在 PUSH0 的情况下并未定义 size
                }
                POP => {
                    self.pop();
                }
                _ => {
                    // 处理其他未覆盖到的操作
                }
            }
            
            // if PUSH1 <= op && op <= PUSH32  {
            //     let size:usize = (op-PUSH1+1) as usize;
            //     self.push(size)
            // }else if PUSH0 == op {
            //     self.stack.push(0);
            //     // self.pc += size;
            // }else if POP == op {
            //     self.pop();
            // }
        }
    }
    
    ///堆栈行为
    /// 出栈
    fn pop(&mut self){
        if self.stack.len()!=0 {
            self.stack.pop();
        }else {
            warn!("栈空无法调用pop方法");
        }
    }

    ///入栈
    fn push(&mut self, size:usize) {
        let ops:Vec<u8> = self.code[self.pc..self.pc+size].to_vec();
        ops.iter().for_each(|&x|{
            let value:u32 = u32::from_str_radix(x.to_string().as_str(), 16).unwrap();
            self.stack.push(value);
        });
        self.pc += size
    }

    // ///算数指令
    // /// add
    // fn add(){

    // }

}

fn init_log(){
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
}

#[test]
fn test(){
    let bytes = vec![0x60, 0x01, 0x60, 0x01,0x50];
    let mut evm_test = Evm::new(bytes);
    println!("{:?}", evm_test);
    evm_test.run();
    println!("{:?}",evm_test);    
}