// 栈堆指令
pub const PUSH0: u8 = 0x5F;
pub const PUSH1: u8 = 0x60;
pub const PUSH32: u8 = 0x7F;
pub const POP: u8 = 0x50;
// 算数指令
pub const ADD: u8 = 0x01;
pub const MUL: u8 = 0x02;
pub const SUB: u8 = 0x03;
pub const DIV: u8 = 0x04;
pub const SDIV: u8 = 0x05;
pub const MOD: u8 = 0x06;
pub const SMOD: u8 = 0x07;
pub const ADDMOD: u8 = 0x08;
pub const MULMOD: u8 = 0x09;
pub const EXP: u8 = 0x0A;
pub const SIGNEXTEND: u8 = 0x0B;

// 比较指令
pub const LT: u8 = 0x10;
pub const GT: u8 = 0x11;
pub const SLT: u8 = 0x12;
pub const SGT: u8 = 0x13;
pub const EQ: u8 = 0x14;
pub const ISZERO: u8 = 0x15;

// 位运算指令
pub const AND: u8 = 0x16;
pub const OR: u8 = 0x17;
pub const XOR: u8 = 0x18;
pub const NOT: u8 = 0x19;
pub const BYTE: u8 = 0x1A;
pub const SHL: u8 = 0x1B;
pub const SHR: u8 = 0x1C;
pub const SAR: u8 = 0x1D;

//内存指令
pub const MLOAD: u8 = 0x51;
pub const MSTORE: u8 = 0x52;
pub const MSTORE8: u8 = 0x53;
pub const MSIZE: u8 = 0x59;

// 存储指令
pub const SLOAD: u8 = 0x54;
pub const SSTORE: u8 = 0x55;

// 控制指令
pub const JUMP: u8 = 0x56;
pub const JUMPI: u8 = 0x57;
pub const PC: u8 = 0x58;
pub const JUMPDEST: u8 = 0x5B;
pub const STOP: u8 = 0x00;

// 区块链信息指令
pub const BLOCKHASH: u8 = 0x40;
pub const COINBASE: u8 = 0x41;
pub const TIMESTAMP: u8 = 0x42;
pub const NUMBER: u8 = 0x43;
pub const PREVRANDAO: u8 = 0x44;
pub const GASLIMIT: u8 = 0x45;
pub const CHAINID: u8 = 0x46;
pub const SELFBALANCE: u8 = 0x47;
pub const BASEFEE: u8 = 0x48;

// 堆栈指令
pub const DUP1: u8 = 0x80;
pub const DUP16: u8 = 0x8F;
pub const SWAP1: u8 = 0x90;
pub const SWAP16: u8 = 0x9B;

// sha3 指令
pub const SHA3: u8 = 0x20;

// 账户指令
pub const BALANCE:u8 = 0x31;
pub const EXTCODESIZE:u8 = 0x3B;
pub const EXTCODECOPY:u8 = 0x3C;
pub const EXTCODEHASH:u8 = 0x3F;

// 交易指令
pub const ADDRESS:u8 = 0x30;
pub const ORIGIN:u8 = 0x32;
pub const CALLER:u8 = 0x33;
pub const CALLVALUE:u8 = 0x34;
pub const CALLDATALOAD:u8 = 0x35;
pub const CALLDATASIZE:u8 = 0x36;
pub const CALLDATACOPY:u8 = 0x37;
pub const CODESIZE:u8 = 0x38;
pub const CODECOPY:u8 = 0x39;
pub const GASPRICE:u8 = 0x3a;

