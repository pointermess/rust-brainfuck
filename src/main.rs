mod brainfuck;
use crate::brainfuck::{
    Interpreter,
    HandleMemory,
    HandleProgram
};

fn main() {
    let mut bf = Interpreter::new();

    let mem = &mut bf.get_memory();

    bf.load_program(">++>+>+++");

    bf.run();

    bf.print_state();

    println!("Hello, world!");
}
