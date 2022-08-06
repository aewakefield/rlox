use std::io;

use crate::value::{Value, ValuePosition, Values};

/// Instruction to carry out a particular operation.
#[derive(Debug, PartialEq)]
pub enum OpCode {
    /// Load a constant for use.
    Constant(ValuePosition),
    /// Return from current function.
    Return,
}

impl OpCode {
    /// Write the instruction in disassembly format.
    fn disassemble(&self, writer: &mut impl io::Write, constants: &Values) -> io::Result<()> {
        match self {
            Self::Constant(position) => {
                let value = constants.get(position);
                let name = "OP_CONSTANT";
                write!(writer, "{name:16} {position:04?} '{value:?}'")
            }
            Self::Return => {
                let name = "OP_RETURN";
                write!(writer, "{name:16}")
            }
        }
    }
}

/// Sequence of bytecode.
#[derive(Default)]
pub struct Chunk {
    /// Sequence of instructions.
    ops: Vec<OpCode>,
    /// Constants used in chunk.
    constants: Values,
    /// Lines the instructions originated from.
    /// Each instruction in `ops` will have the line it came from at the same index in `lines`.
    lines: Vec<usize>,
}

impl Chunk {
    /// Add an instruction to this chunk and records the line of source code this instruction came from.
    pub fn write(&mut self, op_code: OpCode, line: usize) {
        self.ops.push(op_code);
        self.lines.push(line);
    }

    /// Add a constant to this chunk.
    pub fn add_constant(&mut self, value: Value) -> ValuePosition {
        self.constants.write(value)
    }

    /// Write the chunk in disassembly format.
    pub fn disassemble(&self, name: &str, writer: &mut impl io::Write) -> io::Result<()> {
        writeln!(writer, "== {name} ==")?;

        let mut last_line = None;
        for (i, (op, line)) in self.ops.iter().zip(&self.lines).enumerate() {
            write!(writer, "{i:04} ")?;
            if last_line.map_or(false, |last_line| last_line == line) {
                write!(writer, "   | ")?;
            } else {
                write!(writer, "{line:4} ")?;
                last_line = Some(line);
            }
            op.disassemble(writer, &self.constants)?;
            writeln!(writer)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_write_opcode_adds_op_and_line_number() {
        let mut chunk = Chunk::default();
        let line = 42;

        chunk.write(OpCode::Return, line);

        assert_eq!(chunk.ops, vec![OpCode::Return]);
        assert_eq!(chunk.lines, vec![line]);
    }

    #[test]
    fn chunk_disassemble_when_empty_prints_title() {
        let chunk = Chunk::default();
        let name = "empty chunk";
        let mut output = Vec::new();

        chunk.disassemble(name, &mut output).unwrap();

        let output = String::from_utf8(output).unwrap();
        assert_eq!(output, "== empty chunk ==\n");
    }

    #[test]
    fn chunk_disassemble_with_ops_prints_name_and_ops() {
        let mut chunk = Chunk::default();
        let position = chunk.add_constant(Value::Number(42.0));
        chunk.write(OpCode::Constant(position), 1);
        let position = chunk.add_constant(Value::Number(1.2));
        chunk.write(OpCode::Constant(position), 1);
        chunk.write(OpCode::Return, 2);
        let name = "test chunk";
        let mut output = Vec::new();

        chunk.disassemble(name, &mut output).unwrap();

        let output = String::from_utf8(output).unwrap();
        assert_eq!(
            output,
            "== test chunk ==\n0000    1 OP_CONSTANT      0 '42'\n0001    | OP_CONSTANT      1 '1.2'\n0002    2 OP_RETURN       \n"
        );
    }
}
