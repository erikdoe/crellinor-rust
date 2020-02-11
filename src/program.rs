use serde_derive::*;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Debug, PartialOrd, Ord)]
pub enum Instr {
    NOP,
    TUR,
    TUL,
    MOV,
    EAT,
    JMP,
    JRE,
    BFH,
    BFA,
}


#[derive(Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub enum RingMode {
    Continue,
    Repeat,
    Return,
}
