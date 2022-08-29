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
            #[cfg(feature = "tracing")]
            self.trace(op, &chunk)?;

            match op {
                OpCode::Constant(position) => {
                    let constant = chunk.constant(position);
                    writeln!(self.writer, "{constant:?}")?;
                }
                OpCode::Return => return Ok(()),
            }
        }

        Ok(())
    }

    #[cfg(feature = "tracing")]
    fn trace(&mut self, op: &OpCode, chunk: &Chunk) -> io::Result<()> {
        op.disassemble(&mut self.writer, chunk)?;
        writeln!(&mut self.writer)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::value::Value;

    use super::*;

    #[test]
    fn run_constant_prints_contant() {
        let mut output = Vec::new();
        let mut vm = Vm::new(&mut output);
        let mut chunk = Chunk::default();
        let position = chunk.add_constant(Value::Number(42.0));
        chunk.write(OpCode::Constant(position), 1);

        vm.run(chunk).unwrap();

        let output = String::from_utf8(output).unwrap();
        cfg_if::cfg_if! {
            if #[cfg(feature = "tracing")] {
                assert_eq!(output, "OP_CONSTANT      0 '42'\n42\n");
            } else {
                assert_eq!(output, "42\n");
            }
        }
    }

    #[test]
    fn run_return_finishes_running() {
        let mut output = Vec::new();
        let mut vm = Vm::new(&mut output);
        let mut chunk = Chunk::default();
        chunk.write(OpCode::Return, 1);
        let position = chunk.add_constant(Value::Number(42.0));
        chunk.write(OpCode::Constant(position), 1);

        vm.run(chunk).unwrap();

        let output = String::from_utf8(output).unwrap();
        cfg_if::cfg_if! {
            if #[cfg(feature = "tracing")] {
                assert_eq!(output, "OP_RETURN       \n");
            } else {
                assert_eq!(output, "");
            }
        }
    }
}
