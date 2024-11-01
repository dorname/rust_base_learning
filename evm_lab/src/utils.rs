use crate::{const_var::*, fake_db::AccountDb};
use num_bigint::BigUint;
use std::fmt::format;
use tiny_keccak::{Hasher, Keccak};

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
    bytes
        .iter()
        .map(|byte: &u8| match format!("{:1x}", byte) {
            v if v.len() < 2 => {
                format!("0{}", v)
            }
            v if v.len() >= 2 => {
                // println!("{}", v);
                v
            }
            _ => "err".to_string(),
        })
        .collect::<String>()
}

pub fn keccak256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak::v256();
    let mut output = [0u8; 32];
    hasher.update(data);
    hasher.finalize(&mut output);
    output
}

pub fn println_hex(bytes: Vec<u8>, size: usize) -> String {
    format!("{:0>size$}", vec_to_hex_string(bytes), size = size)
}

pub fn get_instruction_name(op: u8) -> String {
    match op {
        PUSH0 => "PUSH0".to_string(),
        operation if operation >= PUSH1 && operation <= PUSH32 => {
            let num = (operation - PUSH1 + 1) as usize;
            format!("PUSH{}", num)
        }
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
        LT => "LT".to_string(),
        GT => "GT".to_string(),
        SLT => "SLT".to_string(),
        SGT => "SGT".to_string(),
        EQ => "EQ".to_string(),
        ISZERO => "ISZERO".to_string(),
        AND => "AND".to_string(),
        OR => "OR".to_string(),
        XOR => "XOR".to_string(),
        NOT => "NOT".to_string(),
        BYTE => "BYTE".to_string(),
        SHL => "SHL".to_string(),
        SHR => "SHR".to_string(),
        SAR => "SAR".to_string(),
        MSTORE => "MSTORE".to_string(),
        MSTORE8 => "MSTORE8".to_string(),
        MSIZE => "MSIZE".to_string(),
        MLOAD => "MLOAD".to_string(),
        SLOAD => "SLOAD".to_string(),
        SSTORE => "SSTORE".to_string(),
        JUMP => "JUMP".to_string(),
        JUMPI => "JUMPI".to_string(),
        PC => "PC".to_string(),
        JUMPDEST => "JUMPDEST".to_string(),
        STOP => "STOP".to_string(),
        BLOCKHASH => "BLOCKHASH".to_string(),
        COINBASE => "COINBASE".to_string(),
        TIMESTAMP => "TIMESTAMP".to_string(),
        NUMBER => "NUMBER".to_string(),
        PREVRANDAO => "PREVRANDAO".to_string(),
        GASLIMIT => "GASLIMIT".to_string(),
        CHAINID => "CHAINID".to_string(),
        SELFBALANCE => "SELFBALANCE".to_string(),
        BASEFEE => "BASEFEE".to_string(),
        operation if operation >= DUP1 && operation <= DUP16 => {
            let num = (operation - DUP1 + 1) as usize;
            format!("DUP{}", num)
        }
        operation if operation >= SWAP1 && operation <= SWAP16 => {
            let num = (operation - SWAP1 + 1) as usize;
            format!("SWAP{}", num)
        }
        SHA3 => "SHA3".to_string(),
        BALANCE => "BALANCE".to_string(),
        EXTCODECOPY => "EXTCODECOPY".to_string(),
        EXTCODEHASH => "EXTCODEHASH".to_string(),
        EXTCODESIZE => "EXTCODESIZE".to_string(),
        operation if operation >= LOG0 && operation <= LOG4 => {
            let num = (operation - LOG0) as usize;
            format!("LOG{}", num)
        }
        RETURN => "RETURN".to_string(),
        RETURNDATACOPY => "RETURNDATACOPY".to_string(),
        RETURNDATASIZE => "RETURNDATASIZE".to_string(),
        REVERT => "REVERT".to_string(),
        INVALID => "INVALID".to_string(),
        CALL => "CALL".to_string(),
        DELEGATECALL => "DELEGATECALL".to_string(),
        STATICCALL => "STATICCALL".to_string(),
        _ => "UNKNOWN".to_string(),
    }
}

pub fn get_account_db() -> AccountDb {
    AccountDb::mock()
}

pub fn get_account_db_for_calltest() -> AccountDb {
    // AccountDb::mock()
    AccountDb::mock_for_calltest()
}
