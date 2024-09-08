
pub const PUSH0:u8 = 0x5F;
pub const PUSH1:u8 = 0x60;
pub const PUSH32:u8 = 0x7F;
pub const POP:u8 = 0x50;
// 算数指令
pub const ADD:u8 = 0x01;
pub const MUL:u8 = 0x02;
pub const SUB:u8 = 0x03;
pub const DIV:u8 = 0x04;    
pub fn getInstructionName(op:u8) -> String{
    match op {
        PUSH0 => "PUSH0".to_string(),
        PUSH1 => "PUSH1".to_string(),
        PUSH32 => "PUSH32".to_string(),
        POP => "POP".to_string(),
        ADD => "ADD".to_string(),
        MUL => "MUL".to_string(),
        SUB => "SUB".to_string(),
        DIV => "DIV".to_string(),
        _ => "UNKNOWN".to_string()
    }
}