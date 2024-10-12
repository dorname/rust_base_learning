use crate::evm::Evm;
use crate::log_utils::*;
use crate::ops::traits::*;
use crate::utils::*;
use num_bigint::BigUint;
use num_traits::{one, zero, ToPrimitive};

impl Memory for Evm {
    fn mload(&mut self) {}
    fn msize(&mut self) {}

    /// 内存写指令
    /// 目前位数与evm.codes模拟的结果有差异
    /// codes总位数是128个十六进制数，一个十六进制数代表4位
    /// 文档则显示是64个十六进制数表示存到内存里面的值，一个十六进制数代表4位
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x02, 0x60, 0x20, 0x52];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn mstore(&mut self) {
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let unit_offset = self.stack.pop().unwrap();
        let unit_value = self.stack.pop().unwrap();
        let mut logger = LogTemplate::new_two_cal(
            "MSTORE".to_owned(),
            "mstore".to_owned(),
            unit_value.clone(),
            unit_offset.clone(),
        );
        logger.log_two_cal();
        let offset = unit_offset.0;
        println!(
            "{},{}",
            offset.clone(),
            BigUint::from(64u8).to_usize().unwrap()
        );
        let value = unit_value.0;
        // 如果内存长度不够，自动扩展
        if self.memory.len() < (offset.clone() + BigUint::from(32u8)).to_usize().unwrap() {
            self.memory.resize(
                (offset.clone() + BigUint::from(32u8)).to_usize().unwrap(),
                0,
            ); // 将不足的部分填充为
        }
        let mut val_bytes = value.to_bytes_be();
        if val_bytes.len() < 32 {
            // 如果字节长度不足 32 字节，前面填充 0
            let padding = vec![0u8; 32 - val_bytes.len()];
            val_bytes = [padding, val_bytes].concat();
        }
        // 将 32 字节数据写入内存中的偏移位置
        for (i, val_byte) in val_bytes.iter().enumerate() {
            self.memory[(offset.clone() + BigUint::from(i)).to_usize().unwrap()] = *val_byte;
        }

        //因为一个十六进制数代表4位所以打印的时候把长度设置成64位长度
        logger.log_memory_store_val(
            self.memory.clone(),
            (offset.clone() + BigUint::from(32u8)).to_usize().unwrap(),
        );
    }
    fn mstore8(&mut self) {
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let unit_offset = self.stack.pop().unwrap();
        let unit_value = self.stack.pop().unwrap();
        let mut logger = LogTemplate::new_two_cal(
            "MSTORE8".to_owned(),
            "mstore8".to_owned(),
            unit_value.clone(),
            unit_offset.clone(),
        );
        logger.log_two_cal();
        let offset = unit_offset.0;
        println!(
            "{},{}",
            offset.clone(),
            BigUint::from(64u8).to_usize().unwrap()
        );
        let value = unit_value.0;
        // 如果内存长度不够，自动扩展
        if self.memory.len() < (offset.clone() + BigUint::from(32u8)).to_usize().unwrap() {
            self.memory.resize(
                (offset.clone() + BigUint::from(32u8)).to_usize().unwrap(),
                0,
            ); // 将不足的部分填充为
        }
        let mask = (BigUint::from(1u8) << 3) - BigUint::from(1u8);
        let low_val:BigUint = value & mask;
        self.memory[offset.clone().to_usize().unwrap()] = low_val.to_u8().unwrap();
        //因为一个十六进制数代表4位所以打印的时候把长度设置成64位长度
        logger.log_memory_store_val(
            self.memory.clone(),
            (offset.clone() + BigUint::from(32u8)).to_usize().unwrap(),
        );
    }
}

#[test]
fn mstore_test() {
    // let excute_codes = "60ff600152";
    let excute_codes = "6002602052";
    // let excute_codes = "61ff02601452";
    let bytes = hex::decode(excute_codes).unwrap();
    let mut evm_test = Evm::new(bytes);
    evm_test.run();
    println!("{:?}", vec_to_hex_string(evm_test.memory));
}

#[test]
fn mstore8_test() {
    let excute_codes = "61ff02600153";
    // let excute_codes = "6002602053";
    let bytes = hex::decode(excute_codes).unwrap();
    let mut evm_test = Evm::new(bytes);
    evm_test.run();
    println!("{:?}", vec_to_hex_string(evm_test.memory));
}
