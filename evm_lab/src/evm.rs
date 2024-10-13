use std::collections::HashMap;

use crate::const_var::*;
use crate::ops::traits::*;
use crate::utils::*;
use hex::decode;
use log::*;
use num_bigint::BigUint;
use num_traits::{ToPrimitive, Zero};

#[derive(Debug)]
pub struct Evm {
    //以太坊虚拟机字节码
    code: Vec<u8>,
    //程序计数器
    pub pc: usize,
    //堆栈
    //每个元素长度为256位（32字节），最大深度为1024元素，但是每个操作只能操作堆栈顶的16个元素
    pub stack: Vec<(BigUint, u8)>,
    //存储
    pub storage: HashMap<BigUint, (BigUint, u8)>,
    //内存
    pub memory: Vec<u8>,
    // 有效指令
    pub valid_jumpdest: HashMap<usize,bool>,
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
    pub fn new(code: Vec<u8>) -> Self {
        init_log();
        
        // 初始化valid_jumpdest
        // let mut vaild_jumpdest: HashMap<usize, bool> = HashMap::new();
        let valid_jumpdest = code.iter().fold((HashMap::<usize,bool>::new(),0 as usize),|(mut mp,idx),&val|{
            let mut step = 1 as usize;
            if val == JUMPDEST {
                mp.insert(idx, true);
            }
            if PUSH1 <= val && val <= PUSH32 {
                step = (val - PUSH1 + 1) as usize;
            }
            (mp,idx + step)
        }).0;

        Evm {
            code: code,
            pc: 0,
            stack: Vec::<(BigUint, u8)>::new(),
            memory: Vec::<u8>::new(),
            storage: HashMap::new(),
            valid_jumpdest: valid_jumpdest,
        }
    }

    /// 获取当前待执行的指令
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x01, 0x60, 0x01,0x50];
    /// let mut evm_test = Evm::new(bytes);
    /// let op:u8 = evm_test.get_current_instruction();
    /// ```
    pub fn get_current_instruction(&mut self) -> u8 {
        let &op: &u8 = self.code.get(self.pc).unwrap();
        info!("当前的程序计数器为{}", self.pc);
        info!("当前执行的指令为{}", get_instruction_name(op));
        //程序计数器累加，代表当前指令已取出并准备执行，计数器指向下一个指令。
        self.pc += 1;
        info!(
            "下一个程序计数器值:{}(获取当前指令后,程序计数器指向下一个元素索引故pc+1)",
            self.pc
        );
        return op.clone();
    }

    /// 执行所有指令
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x01, 0x60, 0x01,0x50];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    pub fn run(&mut self) {
        while self.pc < self.code.len() {
            let op: u8 = self.get_current_instruction();

            match op {
                op if (PUSH1 <= op && op <= PUSH32) => {
                    let size = (op - PUSH1 + 1) as usize;
                    self.push(size);
                }
                PUSH0 => {
                    self.stack.push((BigUint::from(0u32), 0u8));
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
                MOD => {
                    self.n_mod();
                }
                SMOD => {
                    self.smod();
                }
                ADDMOD => {
                    self.add_mod();
                }
                MULMOD => {
                    self.mul_mod();
                }
                EXP => {
                    self.exp();
                }
                SIGNEXTEND => {
                    self.sign_extend();
                }
                LT => {
                    self.lt();
                }
                GT => {
                    self.gt();
                }
                SLT => {
                    self.slt();
                }
                SGT => {
                    self.sgt();
                }
                EQ => {
                    self.eq();
                }
                ISZERO => {
                    self.is_zero();
                }
                AND => {
                    self.and();
                }
                OR => {
                    self.or();
                }
                XOR => {
                    self.xor();
                }
                NOT => {
                    self.not();
                }
                BYTE => {
                    self.byte();
                }
                SHL => {
                    self.shl();
                }
                SHR => {
                    self.shr();
                }
                SAR => {
                    self.sar();
                }
                MSTORE => {
                    self.mstore();
                }
                MSTORE8 => {
                    self.mstore8();
                }
                MSIZE => {
                    self.msize();
                }
                MLOAD => {
                    self.mload();
                }
                SSTORE => {
                    self.sstore();
                }
                SLOAD => {
                    self.sload();
                }
                STOP => {
                    info!("stop");
                    break;
                }
                JUMPDEST => {
                    self.jumpdest();
                }
                JUMP => {
                    self.jump();
                }
                JUMPI => {
                    self.jumpi();
                }
                PC => {
                    self.pc();
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
    pub fn pop(&mut self) {
        if self.stack.len() != 0 {
            self.stack.pop();
        } else {
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
    pub fn push(&mut self, size: usize) {
        let ops: Vec<u8> = self.code[self.pc..self.pc + size].to_vec();
        let result = ops
            .iter()
            .fold((ops.len() - 1, BigUint::zero()), |(pos, mut sum), &x| {
                let mut value: u32 = u32::from_str_radix(x.to_string().as_str(), 16).unwrap();
                if x > 0x09 {
                    value = x.clone() as u32;
                }
                sum += BigUint::from(value) << (8 * pos);
                if pos == 0 {
                    return (pos, sum);
                }
                (pos - 1, sum)
                //
            })
            .1;
        info!("PUSH的值为:{}", vec_to_hex_string(result.to_radix_be(16)));
        self.stack.push((result, 0u8));
        // 入栈时程序计数器累加，size为入栈元素的个数
        info!("程序计数器:{}(将size个元素入栈，pc+size)", self.pc + size);
        self.pc += size
    }
    pub fn fill_memory(&mut self) {
        // 获取当前内存长度
        let current_len = self.memory.len();

        // 计算到32的下一个倍数需要的字节数
        let padding_needed = (32 - (current_len % 32)) % 32;

        // 如果需要填充，则扩展内存
        if padding_needed > 0 {
            // 扩展内存，将内存填充到32的倍数
            self.memory.resize(current_len + padding_needed, 0);
        }
    }
}

fn init_log() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
}

#[test]
fn test_push() {
    let excute_codes = "62ff0011";
    let bytes = hex::decode(excute_codes).unwrap();
    // let bytes = vec![0x61, 0xff,0x00];
    let mut evm_test = Evm::new(bytes);
    evm_test.run();
    println!("{:?}", evm_test.stack);
}

#[test]
fn test_idx() {
    let mut value: u32 = u32::from_str_radix(0xff.to_string().as_str(), 16).unwrap();
    if 0xff > 0x09 {
        value = 0xff.clone() as u32;
    }
    let result: BigUint = BigUint::from(value) << 8;
    println!("PUSH的值为:{}", vec_to_hex_string(result.to_radix_be(16)));
}
