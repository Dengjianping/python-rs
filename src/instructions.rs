#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Opcode {
    HLT,
    IGL
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::HLT,
            _ => Self::IGL,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Instruction {
    opcode: Opcode
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Self {
        Instruction {
            opcode
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn test_create_instructions() {
        let instrcution = Instruction::new(Opcode::HLT);
        assert_eq!(instrcution.opcode, Opcode::HLT);
    }
}
