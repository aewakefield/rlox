use std::{fmt, io};

/// Instruction to carry out a particular operation.
#[derive(PartialEq)]
pub enum OpCode {
    /// Return from current function.
    Return,
}

impl fmt::Debug for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Return => write!(f, "OP_RETURN"),
        }
    }
}

/// Sequence of bytecode.
#[derive(Default)]
pub struct Chunk {
    ops: Vec<OpCode>,
}

impl Chunk {
    /// Add an instruction to this chunk.
    pub fn write(&mut self, op_code: OpCode) {
        self.ops.push(op_code);
    }

    /// Write the chunk in dissassembly format to the writer.
    pub fn dissassemble(&self, name: &str, writer: &mut impl io::Write) -> io::Result<()> {
        write!(writer, "== {name} ==\n{self:?}")
    }
}

impl fmt::Debug for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, op) in self.ops.iter().enumerate() {
            writeln!(f, "{i:04} {op:?}")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_write_opcode_adds_op() {
        let mut chunk = Chunk::default();

        chunk.write(OpCode::Return);

        assert_eq!(chunk.ops, vec![OpCode::Return]);
    }

    #[test]
    fn chunk_dissassemble_when_empty_prints_title() {
        let chunk = Chunk::default();
        let name = "empty chunk";
        let mut output = Vec::new();

        chunk.dissassemble(name, &mut output).unwrap();

        let output = String::from_utf8(output).unwrap();
        assert_eq!(output, "== empty chunk ==\n");
    }

    #[test]
    fn chunk_dissassemble_with_ops_prints_name_and_ops() {
        let mut chunk = Chunk::default();
        chunk.write(OpCode::Return);
        chunk.write(OpCode::Return);
        let name = "test chunk";
        let mut output = Vec::new();

        chunk.dissassemble(name, &mut output).unwrap();

        let output = String::from_utf8(output).unwrap();
        assert_eq!(output, "== test chunk ==\n0000 OP_RETURN\n0001 OP_RETURN\n");
    }
}
