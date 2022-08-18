mod chunk;
mod value;
mod vm;

use std::io;

use chunk::{Chunk, OpCode};
use value::Value;
use vm::{Vm, VmError};

fn main() -> Result<(), VmError> {
    let mut stdout = io::stdout();

    let mut chunk = Chunk::default();
    let position = chunk.add_constant(Value::Number(42.0));
    chunk.write(OpCode::Constant(position), 1);
    let position = chunk.add_constant(Value::Number(1.2));
    chunk.write(OpCode::Constant(position), 1);
    chunk.write(OpCode::Return, 2);

    chunk.disassemble("test chunk", &mut stdout)?;

    let mut vm = Vm::new(&mut stdout);
    vm.run(chunk)
}
