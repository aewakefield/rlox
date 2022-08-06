mod chunk;
mod value;

use std::io;

use chunk::{Chunk, OpCode};
use value::Value;

fn main() -> io::Result<()> {
    let mut chunk = Chunk::default();
    let position = chunk.add_constant(Value::Number(42.0));
    chunk.write(OpCode::Constant(position), 1);
    let position = chunk.add_constant(Value::Number(1.2));
    chunk.write(OpCode::Constant(position), 1);
    chunk.write(OpCode::Return, 2);

    chunk.disassemble("test chunk", &mut io::stdout())
}
