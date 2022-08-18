use std::io;

use crate::chunk::{Chunk, OpCode};

pub struct Vm<W> {
    writer: W,
}

#[derive(thiserror::Error, Debug)]
pub enum VmError {
    #[error(transparent)]
    IoError(#[from] io::Error),
}

impl<W: io::Write> Vm<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    pub fn run(&mut self, chunk: Chunk) -> Result<(), VmError> {
        for op in chunk.ops() {
            match op {
                OpCode::Constant(position) => {
                    let constant = chunk.constant(position);
                    writeln!(self.writer, "{constant:?}")?;
                }
                OpCode::Return => writeln!(self.writer, "OP_RETURN")?,
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::value::Value;

    use super::*;

    #[test]
    fn run_constant() {
        let mut output = Vec::new();
        let mut vm = Vm::new(&mut output);
        let mut chunk = Chunk::default();
        let position = chunk.add_constant(Value::Number(42.0));
        chunk.write(OpCode::Constant(position), 1);
        chunk.write(OpCode::Return, 1);

        vm.run(chunk).unwrap();

        let output = String::from_utf8(output).unwrap();
        assert_eq!(output, "42\nOP_RETURN\n");
    }
}
