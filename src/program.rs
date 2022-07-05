use serde_derive::*;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Debug, PartialOrd, Ord)]
pub enum Instr {
    NOP,
    TUR,
    TUL,
    MOV,
    EAT,
    JMP,
    JMZ,
    BFH,
    BFA,
}
