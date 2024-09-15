extern crate num_bigint;
extern crate num_integer;
extern crate num_traits;
use crate::const_var::*;
use crate::log_utils::*;
use log::*;
use num_bigint::{BigInt, BigUint, Sign};
use num_integer::Integer;
use num_traits::{zero, One, ToPrimitive, Zero};
#[derive(Debug)]
pub struct Evm {
    //以太坊虚拟机字节码
    code: Vec<u8>,
    //程序计数器
    pc: usize,
    //堆栈
    //每个元素长度为256位（32字节），最大深度为1024元素，但是每个操作只能操作堆栈顶的16个元素
    stack: Vec<(BigUint, u8)>,
}
/// 算术指令集特征
pub trait arithmetic {
    // 算术指令方法签名
     fn add(&mut self);
     fn mul(&mut self);
     fn sub(&mut self);
     fn div(&mut self);
     fn sdiv(&mut self);
     fn n_mod(&mut self);
     fn smod(&mut self);
     fn add_mod(&mut self);
     fn mul_mod(&mut self);
     fn exp(&mut self);
     fn sign_extend(&mut self);
}

impl arithmetic for Evm {

    /// add
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x02, 0x60, 0x03,0x01];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn add(&mut self) {
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let unit_a = self.stack.pop().unwrap();
        let unit_b = self.stack.pop().unwrap();
        let mut logger = LogTemplate::new_two_cal(
            "ADD".to_owned(),
            "+".to_owned(),
            unit_a.clone(),
            unit_b.clone(),
        );
        logger.log_two_cal();
        let sign_a = unit_a.1;
        let sign_b = unit_b.1;
        let a = get_uint256(unit_a);
        let b = get_uint256(unit_b);
        // !(1 ^ 1)=1 !(0 ^ 0)=1 !(1 ^ 0)=0 !(0 ^ 1)=0
        // true 同号 为1 false 异号 为0
        let same_sign = !((sign_a ^ sign_b) != 0) as u8;
        let sign_a_b = if same_sign == 1u8 && sign_a == 1u8 {
            1u8
        } else {
            0u8
        };
        let result: BigUint = match same_sign {
            1u8 => {
                let mut add_result: BigUint =
                    (a.clone() + b.clone()) % (BigUint::from(1u32) << 256); //加法结果需要模2^256，防止溢出\
                if sign_a_b == 1 {
                    add_result = (BigUint::from(1u32) << 256) - add_result;
                }
                add_result
            }
            _ => {
                // 1 -2
                if a < b {
                    let add_result: BigUint = (BigUint::from(1u32) << 256)
                        - ((b.clone() - a.clone()) % (BigUint::from(1u32) << 256)); // 加法结果需要模2^256，防止溢出
                    add_result
                } else {
                    // 2 -1
                    let add_result: BigUint =
                        (a.clone() - b.clone()) % (BigUint::from(1u32) << 256); // 加法结果需要模2^256，防止溢出
                    add_result
                }
            }
        };
        logger.set_result(result.clone());
        logger.set_is_negative(sign_a_b.clone());
        logger.log_store_val();
        logger.log_real_val();
        self.stack.push((result, sign_a_b));
    }

    /// 乘法指令
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x02, 0x60, 0x03,0x02];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn mul(&mut self) {
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let unit_a = self.stack.pop().unwrap();
        let unit_b = self.stack.pop().unwrap();
        let mut logger = LogTemplate::new_two_cal(
            "MUL".to_owned(),
            "*".to_owned(),
            unit_a.clone(),
            unit_b.clone(),
        );
        logger.log_two_cal();
        let sign_a = unit_a.1;
        let sign_b = unit_b.1;
        let a = get_uint256(unit_a);
        let b = get_uint256(unit_b);
        let mut result: BigUint = (a * b) % (BigUint::from(1u32) << 256);
        // !(1 ^ 1)=1 !(0 ^ 0)=1 !(1 ^ 0)=0 !(0 ^ 1)=0
        // true 同号 为1 false 异号 为0
        let same_sign = !((sign_a ^ sign_b) != 0) as u8;
        if same_sign == 0u8 {
            result = (BigUint::from(1u32) << 256) - result;
        }
        logger.set_result(result.clone());
        logger.set_is_negative(same_sign);
        logger.log_store_val();
        logger.log_real_val();
        self.stack.push((result, same_sign));
    }

    /// 减法指令
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x04, 0x60, 0x03,0x03];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn sub(&mut self) {
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let unit_a = self.stack.pop().unwrap();
        let unit_b = self.stack.pop().unwrap();
        let sign_a = unit_a.1;
        let sign_b = unit_b.1;
        let mut logger = LogTemplate::new_two_cal(
            "SUB".to_owned(),
            "-".to_owned(),
            unit_a.clone(),
            unit_b.clone(),
        );
        logger.log_two_cal();
        // true 同号 为1 false 异号 为0
        let same_sign = !((sign_a ^ sign_b) != 0) as u8;
        let a = get_uint256(unit_a);
        let b = get_uint256(unit_b);
        let sign_a_b = if same_sign == 1 && a.clone() < b.clone() || same_sign == 0 && sign_a == 1 {
            1u8
        } else {
            0u8
        };
        let result: BigUint = match same_sign {
            1u8 => {
                // -1 - -2
                // 1 - 2
                if a < b {
                    (BigUint::from(1u32) << 256) - ((b - a) % (BigUint::from(1u32) << 256))
                } else {
                    (a - b) % (BigUint::from(1u32) << 256)
                }
            }
            _ => {
                //-1 - 2
                // 1 - - 2
                // 2 - -1
                (BigUint::from(1u32) << 256) - ((a + b) % (BigUint::from(1u32) << 256))
            }
        };
        logger.set_result(result.clone());
        logger.set_is_negative(sign_a_b);
        logger.log_store_val();
        logger.log_real_val();
        self.stack.push((result, sign_a_b));
    }

    /// 除法指令
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x06, 0x60, 0x03,0x04];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn div(&mut self) {
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let unit_a = self.stack.pop().unwrap();
        let unit_b = self.stack.pop().unwrap();
        let mut logger = LogTemplate::new_two_cal(
            "DIV".to_owned(),
            "/".to_owned(),
            unit_a.clone(),
            unit_b.clone(),
        );
        logger.log_two_cal();
        let a = get_uint256(unit_a);
        let b = get_uint256(unit_b);
        if b == BigUint::from(0u32) {
            panic!("Division by zero");
        }
        let result: BigUint = (a / b) % (BigUint::from(1u32) << 256);
        logger.set_result(result.clone());
        logger.set_is_negative(0u8);
        logger.log_store_val();
        logger.log_real_val();
        self.stack.push((result, 0u8));
    }

    ///带符号除法运算
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x02, 0x60, 0x03,0x05];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn sdiv(&mut self) {
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let unit_a = self.stack.pop().unwrap();
        let unit_b = self.stack.pop().unwrap();
        let mut logger = LogTemplate::new_two_cal(
            "SDIV".to_owned(),
            "/".to_owned(),
            unit_a.clone(),
            unit_b.clone(),
        );
        logger.log_two_cal();
        let sign_a = unit_a.1;
        let sign_b = unit_b.1;
        let a = get_uint256(unit_a);
        let b = get_uint256(unit_b);
        // true 同号 为1 false 异号 为0
        let same_sign = !((sign_a ^ sign_b) != 0) as u8;
        if b == BigUint::from(0u32) {
            panic!("Division by zero");
        }
        let mut result: BigUint = (a / b) % (BigUint::from(1u32) << 256);
        let sign_a_b = if same_sign == 1 {
            0u8
        } else {
            result = (BigUint::from(1u32) << 256) - result;
            1u8
        };
        logger.set_result(result.clone());
        logger.set_is_negative(sign_a_b);
        logger.log_store_val();
        logger.log_real_val();
        self.stack.push((result, sign_a_b));
    }

    /// 取模指令
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x02, 0x60, 0x03,0x06];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn n_mod(&mut self) {
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let unit_a = self.stack.pop().unwrap();
        let unit_b = self.stack.pop().unwrap();
        let mut logger = LogTemplate::new_two_cal(
            "MOD".to_owned(),
            "%".to_owned(),
            unit_a.clone(),
            unit_b.clone(),
        );
        logger.log_two_cal();
        let a = get_uint256(unit_a);
        let b = get_uint256(unit_b);
        if b == zero() {
            panic!("Mod by zero");
        }
        let result = a % b;
        logger.set_result(result.clone());
        logger.set_is_negative(0u8);
        logger.log_store_val();
        logger.log_real_val();
        self.stack.push((result, 0u8));
    }

    /// 带符号取模运算
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x02, 0x60, 0x03,0x07];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn smod(&mut self) {
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let unit_a = self.stack.pop().unwrap();
        let unit_b = self.stack.pop().unwrap();
        let mut logger = LogTemplate::new_two_cal(
            "SMOD".to_owned(),
            "%".to_owned(),
            unit_a.clone(),
            unit_b.clone(),
        );
        logger.log_two_cal();
        let sign_a = unit_a.1;
        let sign_b = unit_b.1;
        let a = get_uint256(unit_a);
        let b = get_uint256(unit_b);
        // true 同号 为1 false 异号 为0
        let same_sign = !((sign_a ^ sign_b) != 0) as u8;
        if b == zero() {
            panic!("Smod by zero");
        }
        let mut result = a % b;
        let mut is_negative = 0u8;
        if same_sign == 0u8 {
            result = (BigUint::from(1u32) << 256) - result;
            is_negative = 1u8;
        }
        logger.set_result(result.clone());
        logger.set_is_negative(is_negative);
        logger.log_store_val();
        logger.log_real_val();
        self.stack.push((result, is_negative));
    }

    /// 加法取模运算
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x02, 0x60, 0x03,0x60,0x03,0x08];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn add_mod(&mut self) {
        if self.stack.len() < 3 {
            panic!("Stack underflow");
        }
        let unit_a = self.stack.pop().unwrap();
        let unit_b = self.stack.pop().unwrap();
        let unit_c = self.stack.pop().unwrap();
        let mut logger = LogTemplate::new_three_cal(
            "ADDMOD".to_owned(),
            "+".to_owned(),
            unit_a.clone(),
            unit_b.clone(),
            unit_c.clone(),
        );
        logger.log_three_cal();
        let sign_c = unit_c.1;
        let a = get_uint256(unit_a);
        let b = get_uint256(unit_b);
        let c = get_uint256(unit_c);
        if c.is_zero() {
            panic!("Mod by Zero");
        }
        // if a+b > 0
        let sign_a_b = match a.clone() + b.clone() {
            v if v >= zero() => 0u8,
            _ => 1u8,
        };
        let mut result: BigUint = (a + b) % c;
        // true 同号 为1 false 异号 为0
        let same_sign = !((sign_a_b ^ sign_c) != 0) as u8;
        let mut is_negative = 0u8;
        if same_sign == 0u8 {
            is_negative = 1u8;
            result = (BigUint::from(1u32) << 256) - result;
        };
        logger.set_result(result.clone());
        logger.set_is_negative(is_negative);
        logger.log_store_val();
        logger.log_real_val();
        self.stack.push((result, is_negative));
    }

    /// 乘法取模指令
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x02, 0x60, 0x03,0x60,0x03,0x09];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn mul_mod(&mut self) {
        if self.stack.len() < 3 {
            panic!("Stack underflow");
        }
        let unit_a = self.stack.pop().unwrap();
        let unit_b = self.stack.pop().unwrap();
        let unit_c = self.stack.pop().unwrap();
        let mut logger = LogTemplate::new_three_cal(
            "MULMOD".to_owned(),
            "*".to_owned(),
            unit_a.clone(),
            unit_b.clone(),
            unit_c.clone(),
        );
        logger.log_three_cal();
        let sign_a = unit_a.1;
        let sign_b = unit_b.1;
        let sign_c = unit_c.1;

        let a = get_uint256(unit_a);
        let b = get_uint256(unit_b);
        let c = get_uint256(unit_c);
        if c.is_zero() {
            panic!("Mod by Zero");
        }
        let sign_a_b_c = ((sign_a | sign_b | sign_c) != 0) as u8;
        let mut mul_mod_result = (a.clone() * b.clone()) % c.clone();
        if sign_a_b_c != 0 {
            mul_mod_result = (BigUint::from(1u32) << 256) - mul_mod_result;
        }
        logger.set_result(mul_mod_result.clone());
        logger.set_is_negative(sign_a_b_c);
        logger.log_store_val();
        logger.log_real_val();
        self.stack.push((mul_mod_result, sign_a_b_c));
    }

    /// 指数运算指令
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x02, 0x60, 0x03,0x0a];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn exp(&mut self) {
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let unit_a = self.stack.pop().unwrap();
        let unit_b = self.stack.pop().unwrap();
        let mut logger = LogTemplate::new_two_cal(
            "EXP".to_owned(),
            "^".to_owned(),
            unit_a.clone(),
            unit_b.clone(),
        );
        logger.log_two_cal();
        let a = get_uint256(unit_a);
        let b = get_uint256(unit_b);
        let result = if b.is_zero() {
            BigUint::from(0u8)
        } else {
            a.modpow(&b, &(BigUint::from(1u32) << 256))
        };
        logger.set_result(result.clone());
        logger.set_is_negative(0u8);
        logger.log_store_val();
        logger.log_real_val();
        self.stack.push((result, 0u8));
    }

    /// 符号位扩展指令
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x02, 0x60, 0x03,0x0b];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn sign_extend(&mut self) {
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let unit_b = self.stack.pop().unwrap();
        let unit_x = self.stack.pop().unwrap();
        let mut logger = LogTemplate::new_two_cal(
            "SIGNEXTEND".to_owned(),
            "extend".to_owned(),
            unit_b.clone(),
            unit_x.clone(),
        );
        logger.log_two_cal();
        let b = get_uint256(unit_b);
        let x = get_uint256(unit_x);
        if b < BigUint::from(32u8) {
            // 将 b 转换为 u32
            let b_u32 = b.to_u32().unwrap();

            // 计算位索引：bit_index = 8 * b + 7
            let bit_index = 8u32 * b_u32 + 7;

            // 创建符号位掩码：sign_bit = 1 << bit_index
            let sign_bit = BigUint::one() << bit_index;

            // 创建值掩码：mask = (1 << (bit_index + 1)) - 1
            let mask = (BigUint::one() << (bit_index + 1)) - BigUint::one();

            // 创建全 1 掩码：full_mask = 2^256 - 1
            let full_mask = (BigUint::one() << 256) - BigUint::one();

            // 计算 not_mask = full_mask ^ mask
            let not_mask = &full_mask ^ &mask;

            // 提取有效位
            let value = &x & &mask;

            // 检查符号位是否为 1
            let is_negative = (&x & &sign_bit) != BigUint::zero();

            // 执行符号扩展
            let result: BigInt = if is_negative {
                // 负数，需要将高位全部置为 1
                let extended: BigInt = BigInt::from_biguint(Sign::Minus, not_mask | value);
                // 确保结果在 256 位范围内
                let modulus: BigInt = BigInt::from(1u8) << 256;

                (extended + &modulus).mod_floor(&modulus)
            } else {
                // 正数，保持不变
                BigInt::from_biguint(Sign::Plus, value)
            };

            // 将结果转换回 BigUint
            let result_uint = match result.to_biguint() {
                Some(res) => res,
                None => panic!("Failed to convert result to BigUint"),
            };

            logger.set_result(result_uint.clone());
            logger.set_is_negative(is_negative as u8);
            logger.log_store_val();
            logger.log_real_val();
            // 将结果压回栈中
            self.stack
                .push((result_uint, if is_negative { 1u8 } else { 0u8 }));
        } else {
            logger.set_result(x.clone());
            logger.set_is_negative(0u8);
            logger.log_store_val();
            logger.log_real_val();
            self.stack.push((x, 0u8));
        }
    }
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
        Evm {
            code: code,
            pc: 0,
            stack: Vec::<(BigUint, u8)>::new(),
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
        info!("当前执行的指令为{}", get_instruction_name(op));
        //程序计数器累加，代表当前指令已取出并准备执行，计数器指向下一个指令。
        self.pc += 1;
        info!(
            "程序计数器:{}(获取当前指令后,程序计数器指向下一个元素索引故pc+1)",
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
        ops.iter().for_each(|&x| {
            let mut value: u32 = u32::from_str_radix(x.to_string().as_str(), 16).unwrap();
            if x > 0x09 {
                value = x.clone() as u32;
            }
            info!("PUSH的值为:{}", BigUint::from(value));
            self.stack.push((BigUint::from(value), 0u8));
        });
        // 入栈时程序计数器累加，size为入栈元素的个数
        info!("程序计数器:{}(将size个元素入栈，pc+size)", self.pc + size);
        self.pc += size
    }
   
}

fn init_log() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
}

#[test]
fn add_test() {
    let bytes = vec![0x60, 0x02, 0x60, 0x03, 0x01];
    let mut evm_test = Evm::new(bytes);
    evm_test.run();
    println!("{:?}", evm_test.stack);
}

#[test]
fn mul_test() {
    let bytes = vec![0x60, 0x02, 0x60, 0x03, 0x02];
    let mut evm_test = Evm::new(bytes);
    evm_test.run();
    println!("{:?}", evm_test.stack);
}

#[test]
fn sub_test() {
    let bytes = vec![0x60, 0x04, 0x60, 0x03, 0x03, 0x60, 0x05, 0x01];
    let mut evm_test = Evm::new(bytes);
    evm_test.run();
    println!("{:?}", evm_test.stack);
    // println!("{:?}", get_uint256(evm_test.stack.get(0).unwrap().clone()));
}

#[test]
fn div_test() {
    // let bytes = vec![0x60, 0x06, 0x60, 0x12, 0x04];
    let bytes = vec![0x60, 0x06, 0x60, 0x03, 0x04];

    let mut evm_test = Evm::new(bytes);
    evm_test.run();
    println!("{:?}", evm_test.stack);
}

#[test]
fn sdiv_test() {
    let bytes = vec![0x60, 0x04, 0x60, 0x02, 0x03, 0x60, 0x0b, 0x05];
    let mut evm_test = Evm::new(bytes);
    evm_test.run();
    println!("{:?}", evm_test.stack);
}

#[test]
fn mod_test() {
    let bytes = vec![0x60, 0x04, 0x60, 0x08, 0x06];
    let mut evm_test = Evm::new(bytes);
    evm_test.run();
    println!("{:?}", evm_test.stack);
}

#[test]
fn smod_test() {
    let bytes = vec![0x60, 0x08, 0x60, 0x04, 0x03, 0x60, 0x09, 0x07];
    let mut evm_test = Evm::new(bytes);
    evm_test.run();
    println!("{:?}", evm_test.stack);
}

#[test]
fn add_mod_test() {
    let bytes = vec![0x60, 0x08, 0x60, 0x04, 0x03, 0x60, 0x01, 0x60, 0x09, 0x08];
    // let bytes = vec![0x60, 0x03, 0x60, 0x06, 0x03, 0x60, 0x01, 0x60, 0x09, 0x08];

    let mut evm_test = Evm::new(bytes);
    evm_test.run();
    println!("{:?}", evm_test.stack);
}

#[test]
fn mul_mod_test() {
    let bytes = vec![0x60, 0x08, 0x60, 0x04, 0x03, 0x60, 0x01, 0x60, 0x09, 0x09];
    // let bytes = vec![0x60, 0x03, 0x60, 0x06, 0x03, 0x60, 0x01, 0x60, 0x09, 0x09];

    let mut evm_test = Evm::new(bytes);
    evm_test.run();
    println!("{:?}", evm_test.stack);
}

#[test]
fn exp_test() {
    let bytes = vec![0x60, 0x03, 0x60, 0x02, 0x0a];
    let mut evm_test = Evm::new(bytes);
    evm_test.run();
    println!("{:?}", evm_test.stack);
}

#[test]
fn sign_extend_test() {
    let bytes = vec![0x60, 0x08, 0x60, 0x04, 0x0b];
    let mut evm_test = Evm::new(bytes);
    evm_test.run();
    println!("{:?}", evm_test.stack);
}
