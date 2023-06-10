#[derive(Debug, PartialEq)]
pub enum OpCode {
    HLT,
    IGL,
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: OpCode,
}

impl Instruction {
    pub fn new(opcode: OpCode) -> Instruction {
        Instruction { opcode }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hlt() {
        let opcode = OpCode::HLT;
        assert_eq!(opcode, OpCode::HLT);
    }

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(OpCode::HLT);
        assert_eq!(instruction.opcode, OpCode::HLT);
    }
}
