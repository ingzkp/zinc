use crate::data::types::DataType;
use crate::Instruction;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Program {
    pub input: DataType,
    pub output: DataType,
    pub bytecode: Vec<Instruction>,
}

impl Program {
    pub fn new(input: DataType, output: DataType, bytecode: Vec<Instruction>) -> Self {
        Self {
            input,
            output,
            bytecode,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).expect("Failed to serialize program")
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        bincode::deserialize(bytes).map_err(|e| format!("{:?}", e))
    }
}
