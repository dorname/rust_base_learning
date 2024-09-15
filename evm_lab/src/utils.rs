use crate::const_var::*;
use num_bigint::BigUint;

/// 判断是否是有符号的数据
/// 如果是则返回其补码，否则返回本身
/// ```
/// use evm_lab::evm::Evm;
/// let bytes = vec![0x60, 0x01, 0x60, 0x01,0x50];
/// let mut evm_test = Evm::new(bytes);
/// let unit_x = (BigUint::from(1u32),1u8);
/// let result = evm_test.get_uint256(unit_x);
pub fn get_uint256(unit_x: (BigUint, u8)) -> BigUint {
    match unit_x.1 {
        1 => (BigUint::from(1u32) << 256) - unit_x.0,
        _ => unit_x.0,
    }
}

pub fn vec_to_hex_string(bytes: Vec<u8>) -> String {
    bytes.iter().map(|byte| format!("{:1x}", byte)).collect()
}
pub fn get_instruction_name(op: u8) -> String {
    match op {
        PUSH0 => "PUSH0".to_string(),
        PUSH1 => "PUSH1".to_string(),
        PUSH32 => "PUSH32".to_string(),
        POP => "POP".to_string(),
        ADD => "ADD".to_string(),
        MUL => "MUL".to_string(),
        SUB => "SUB".to_string(),
        DIV => "DIV".to_string(),
        SDIV => "SDIV".to_string(),
        MOD => "MOD".to_string(),
        SMOD => "SMOD".to_string(),
        ADDMOD => "ADDMOD".to_string(),
        MULMOD => "MULMOD".to_string(),
        EXP => "EXP".to_string(),
        SIGNEXTEND => "SIGNEXTEND".to_string(),
        _ => "UNKNOWN".to_string(),
    }
}