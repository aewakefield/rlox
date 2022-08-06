mod chunk;

use std::io;

use chunk::{Chunk, OpCode};

fn main() {
    let mut chunk = Chunk::default();
    chunk.write(OpCode::Return);
    chunk.write(OpCode::Return);

    chunk.dissassemble("test chunk", &mut io::stdout()).unwrap();
}
