extern crate num_bigint;
extern crate num_traits;

use std::panic;
use log::*;
use byteorder::{ByteOrder,BigEndian};
use log4rs::append::rolling_file::policy::compound::trigger::size;

use num_bigint::BigUint;
use num_bigint::ToBigInt;
use num_traits::sign;
use num_traits::{Zero, One};
use crate::const_var::*;
#[derive(Debug)]
pub struct Evm {
    //以太坊虚拟机字节码
    code: Vec<u8>,
    //程序计数器
    pc: usize,
    //堆栈
    //每个元素长度为256位（32字节），最大深度为1024元素，但是每个操作只能操作堆栈顶的16个元素
    stack: Vec<(BigUint,u8)>
}

/// 为虚拟机实现其特征行为和方法
/// 1）初始化方法，
///    字节码为入参
///    计数器初始值为0
///    堆栈初始化为空
impl Evm {
    /// 初始化虚拟机
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x01, 0x60, 0x01,0x50];
    /// let mut evm_test = Evm::new(bytes);
    /// ```
    pub fn new(code:Vec<u8>) -> Self{
        init_log();
        Evm { code: code, pc: 0, stack: Vec::<(BigUint,u8)>::new() }
    }   

    /// 获取当前待执行的指令
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x01, 0x60, 0x01,0x50];
    /// let mut evm_test = Evm::new(bytes);
    /// let op:u8 = evm_test.get_current_instruction();
    /// ```
    pub fn get_current_instruction(&mut self) -> u8 {
        let &op:&u8 = self.code.get(self.pc).unwrap();
        info!("当前执行的指令为{}",get_instruction_name(op));
        //程序计数器累加，代表当前指令已取出并准备执行，计数器指向下一个指令。
        self.pc += 1;
        info!("程序计数器:{}(获取当前指令后,程序计数器指向下一个元素索引故pc+1)",self.pc);
        return op.clone();
    }


    /// 执行所有指令
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x01, 0x60, 0x01,0x50];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    pub fn run(&mut self){
        while self.pc  < self.code.len() {
            let op:u8 = self.get_current_instruction();
            
            match op {
                op if (PUSH1 <= op && op <= PUSH32) => {
                    let size = (op - PUSH1 + 1) as usize;
                    self.push(size);
                }
                PUSH0 => {
                    self.stack.push((BigUint::from(0u32),0u8));
                    // self.pc += size; // 此行应当被删除或者注释掉，因为在 PUSH0 的情况下并未定义 size
                }
                POP => {
                    self.pop();
                }
                ADD => {
                    self.add();
                }
                MUL => {
                    self.mul();
                }
                SUB => {
                    self.sub();
                }
                DIV => {
                    self.div();
                }
                SDIV => {
                    self.sdiv();
                }
                _ => {
                    // 处理其他未覆盖到的操作
                }
            }
        }
    }
    
    /// 堆栈行为
    /// 出栈
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x01, 0x60, 0x01,0x50];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.pop();
    /// ```
    pub fn pop(&mut self){
        if self.stack.len()!=0 {
            self.stack.pop();
        }else {
            warn!("栈空无法调用pop方法");
        }
    }

    /// 入栈
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x01, 0x60, 0x01,0x50];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.push(0 as usize);
    /// ```
    pub fn push(&mut self, size:usize) {
        let ops:Vec<u8> = self.code[self.pc..self.pc+size].to_vec();
        ops.iter().for_each(|&x|{
            let mut value:u32 = u32::from_str_radix(x.to_string().as_str(), 16).unwrap();
            if x > 0x09 {
                value = x.clone() as u32;
            }
            info!("PUSH的值为:{}",BigUint::from(value));
            self.stack.push((BigUint::from(value),0u8));
        });
        // 入栈时程序计数器累加，size为入栈元素的个数
        info!("程序计数器:{}(将size个元素入栈，pc+size)",self.pc+size);
        self.pc += size
    }
    /// 算数指令
    /// add
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x02, 0x60, 0x03,0x01];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    pub fn add(&mut self){
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let unit_a = self.stack.pop().unwrap();
        let sign_a = unit_a.1;
        let a = get_uint256(unit_a);
        let unit_b = self.stack.pop().unwrap();
        let sign_b = unit_b.1;
        let b = get_uint256(unit_b);
        let mut result = BigUint::from(0u8);
        match sign_a==sign_b {
            true => {
                result = (a+b) % (BigUint::from(1u32)<<256);   //加法结果需要模2^256，防止溢出
                info!("ADD:{}",vec_to_hex_string(result.to_radix_be(16)));
                self.stack.push((result,sign_a));
            },
            false => {
                // 1 -2
                if a < b  {
                    result = (BigUint::from(1u32)<<256)-((b-a) % (BigUint::from(1u32)<<256));  // 加法结果需要模2^256，防止溢出
                    info!("ADD:负数{}",vec_to_hex_string(result.to_radix_be(16)));
                    self.stack.push((result,1u8));
                }else {
                // 2 -1
                    result = (a-b) % (BigUint::from(1u32)<<256);  // 加法结果需要模2^256，防止溢出
                    info!("ADD:{}",vec_to_hex_string(result.to_radix_be(16)));
                    self.stack.push((result,0u8));
                }
            }  
        };
    }

    /// 乘法指令
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x02, 0x60, 0x03,0x02];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    pub fn mul(&mut self){
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let unit_a = self.stack.pop().unwrap();
        let sign_a = unit_a.1;
        let a = get_uint256(unit_a);
        let unit_b = self.stack.pop().unwrap();
        let sign_b = unit_b.1;
        let b = get_uint256(unit_b);
        let mut result = BigUint::from(0u8);
        match sign_a==sign_b{
            true => {
                result = (a*b) % (BigUint::from(1u32)<<256);
                info!("MUL:{}",result);
                self.stack.push((result,sign_a));
            },
            _=> {
                result = (a*b) % (BigUint::from(1u32)<<256);
                info!("MUL:{}",result);
                self.stack.push((result,1u8));
            }
        }
    }

    /// 减法指令
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x04, 0x60, 0x03,0x03];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    pub fn sub(&mut self){
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let unit_a = self.stack.pop().unwrap();
        let sign_a = unit_a.1;
        let a = get_uint256(unit_a);
        let unit_b = self.stack.pop().unwrap();
        let sign_b = unit_b.1;
        let b = get_uint256(unit_b);
        let mut result = BigUint::from(0u8);
        match sign_a==sign_b {
            true => {
                // -1 - -2
                // 1 - 2
                if a < b {
                    result = (BigUint::from(1u32)<<256) - ((b-a) % (BigUint::from(1u32)<<256));
                    info!("SUB:负{:?}",vec_to_hex_string(result.to_radix_be(16)));
                    self.stack.push((result,1u8));
                }else {
                    result = (a-b) % (BigUint::from(1u32)<<256);
                    info!("SUB:{:?}",vec_to_hex_string(result.to_radix_be(16)));
                    self.stack.push((result,0u8));
                }
            },
            _=>{
               //-1 - 2
               // 1 - - 2
               // 2 - -1
               result =(BigUint::from(1u32)<<256) -  ((a+b) % (BigUint::from(1u32)<<256));
               if sign_a == 1 {
                    info!("SUB:负{:?}",vec_to_hex_string(result.to_radix_be(16)));
                    self.stack.push((result.clone(),1u8));
               }
               if sign_b == 1 {
                    info!("SUB:{:?}",vec_to_hex_string(result.to_radix_be(16)));
                    self.stack.push((result.clone(),0u8));
               }
            }
        }
    }

    /// 除法指令
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x06, 0x60, 0x03,0x04];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    pub fn div(&mut self){
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let unit_a = self.stack.pop().unwrap();
        let a = get_uint256(unit_a);
        let unit_b = self.stack.pop().unwrap();
        let b = get_uint256(unit_b);
        let mut result = BigUint::from(0u8);
        if b == BigUint::from(0u32) {
            panic!("Division by zero");
        }
        result = (a/b) % (BigUint::from(1u32)<<256);
        info!("DIV:{}",vec_to_hex_string(result.to_radix_be(16)));
        self.stack.push((result,0u8));
    }


    ///带符号除法运算
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x02, 0x60, 0x03,0x05];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    pub fn sdiv(&mut self){
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let unit_a = self.stack.pop().unwrap();
        let sign_a = unit_a.1;
        let a = get_uint256(unit_a);
        let unit_b = self.stack.pop().unwrap();
        let sign_b = unit_b.1;
        let b = get_uint256(unit_b);
        let mut result = BigUint::from(0u8);

        if b == BigUint::from(0u32) {
            panic!("Division by zero");
        }
        match sign_a==sign_b {
            true => {
                result = (a/b) % (BigUint::from(1u32)<<256);
                info!("SDIV:{}",vec_to_hex_string(result.to_radix_be(16)));
                self.stack.push((result,0u8));
            },
            _=> {
                result =(BigUint::from(1u32)<<256) - ((a/b) % (BigUint::from(1u32)<<256));
                info!("SDIV:负{:?}",vec_to_hex_string(result.to_radix_be(16)));
                self.stack.push((result,1u8));
            }
        }
   
    }

}

fn init_log(){
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
}

#[test]
fn add_test(){
    let bytes = vec![0x60, 0x02, 0x60, 0x03,0x01];
    let mut evm_test = Evm::new(bytes);
    evm_test.run();
    println!("{:?}",evm_test.stack);    
}

#[test]
fn mul_test(){
    let bytes = vec![0x60, 0x02, 0x60, 0x03,0x02];
    let mut evm_test = Evm::new(bytes);
    evm_test.run();
    println!("{:?}",evm_test.stack);    
}

#[test]
fn sub_test(){
    let bytes = vec![0x60, 0x04, 0x60, 0x03,0x03,0x60,0x05,0x01];
    let mut evm_test = Evm::new(bytes);
    evm_test.run();
    println!("{:?}",get_uint256(evm_test.stack.get(0).unwrap().clone()));    
}

#[test]
fn div_test(){
    let bytes = vec![0x60, 0x06, 0x60, 0x12,0x04];   
    let mut evm_test = Evm::new(bytes);
    evm_test.run();
    println!("{:?}",evm_test.stack);    
}

#[test]
fn sdiv_test(){
    let bytes = vec![0x60, 0x04, 0x60, 0x02,0x03,0x60,0x0b,0x05];   
    let mut evm_test = Evm::new(bytes);
    evm_test.run();
    println!("{:?}",evm_test.stack);    
}