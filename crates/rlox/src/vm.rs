use std::io;

use crate::{
    chunk::{Chunk, OpCode},
    value::Value,
};

/// Virtual machine to execute instructions.
pub struct Vm<W> {
    /// Writer where output will be written.
    /// If `tracing` feature is enabled tracing information will be written here as well.
    writer: W,
    /// Stack of values storing state of program execution.
    stack: Vec<Value>,
}

/// Error emitted from [`Vm`] when attempting to execute instructions.
#[derive(thiserror::Error, Debug)]
pub enum VmError {
    #[error(transparent)]
    IoError(#[from] io::Error),
}

impl<W: io::Write> Vm<W> {
    /// Create new [`Vm`] that writes to provided writer.
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            stack: Vec::with_capacity(256),
        }
    }

    /// Run [`Chunk`] of instructions.
    pub fn run(&mut self, chunk: Chunk) -> Result<(), VmError> {
        for op in chunk.ops() {
            #[cfg(feature = "tracing")]
            self.trace(op, &chunk)?;

            match op {
                OpCode::Constant(position) => {
                    let constant = chunk.constant(position);
                    self.stack.push(constant.clone());
                }
                OpCode::Return => {
                    let value = self.stack.pop().unwrap_or_default();
                    writeln!(&mut self.writer, "{value:?}")?;

                    return Ok(());
                }
            }
        }

        Ok(())
    }

    /// Write tracing information to writer.
    /// Outputs the current values in stack and then the current instruction.
    #[cfg(feature = "tracing")]
    fn trace(&mut self, op: &OpCode, chunk: &Chunk) -> io::Result<()> {
        write!(&mut self.writer, "          ")?;
        for value in &self.stack {
            write!(&mut self.writer, "[{value:?}]")?;
        }
        writeln!(&mut self.writer)?;

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
    fn run_constant_pushes_to_stack() {
        let mut output = Vec::new();
        let mut vm = Vm::new(&mut output);
        let mut chunk = Chunk::default();
        let value = Value::Number(42.0);
        let position = chunk.add_constant(value.clone());
        chunk.write(OpCode::Constant(position), 1);

        vm.run(chunk).unwrap();

        assert_eq!(vm.stack, vec![value]);
    }

    #[test]
    fn run_return_pops_from_stack() {
        let mut output = Vec::new();
        let mut vm = Vm::new(&mut output);
        let mut chunk = Chunk::default();
        let value = Value::Number(42.0);
        let position = chunk.add_constant(value);
        chunk.write(OpCode::Constant(position), 1);
        chunk.write(OpCode::Return, 1);

        vm.run(chunk).unwrap();

        assert_eq!(vm.stack, vec![]);
    }

    #[test]
    fn run_constant_then_return_writes_value() {
        let mut output = Vec::new();
        let mut vm = Vm::new(&mut output);
        let mut chunk = Chunk::default();
        let value = Value::Number(42.0);
        let position = chunk.add_constant(value);
        chunk.write(OpCode::Constant(position), 1);
        chunk.write(OpCode::Return, 1);

        vm.run(chunk).unwrap();

        let output = String::from_utf8(output).unwrap();
        assert_ends_with(&output, "42\n");
    }

    #[test]
    fn run_return_without_constant_writes_nil() {
        let mut output = Vec::new();
        let mut vm = Vm::new(&mut output);
        let mut chunk = Chunk::default();
        chunk.write(OpCode::Return, 1);

        vm.run(chunk).unwrap();

        let output = String::from_utf8(output).unwrap();
        assert_ends_with(&output, "nil\n");
    }

    #[cfg(feature = "tracing")]
    #[test]
    fn tracing_outputs_stack_and_op() {
        let mut output = Vec::new();
        let mut vm = Vm::new(&mut output);
        let mut chunk = Chunk::default();
        let position = chunk.add_constant(Value::Number(42.0));
        chunk.write(OpCode::Constant(position), 1);
        let position = chunk.add_constant(Value::Number(1.2));
        chunk.write(OpCode::Constant(position), 1);
        chunk.write(OpCode::Return, 1);

        vm.run(chunk).unwrap();

        let output = String::from_utf8(output).unwrap();
        assert_starts_with(&output, "          \nOP_CONSTANT      0 '42'\n          [42]\nOP_CONSTANT      1 '1.2'\n          [42][1.2]\nOP_RETURN       \n");
    }

    fn assert_ends_with(value: &str, ending: &str) {
        assert!(
            value.ends_with(ending),
            "should end with `{ending:?}` got `{value:?}`"
        );
    }

    #[cfg(feature = "tracing")]
    fn assert_starts_with(value: &str, start: &str) {
        assert!(
            value.starts_with(start),
            "should start with `{start:?}` got `{value:?}`"
        )
    }
}
