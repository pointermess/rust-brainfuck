# rust-brainfuck

A simple brainfuck interpreter written in Rust as a fun learning project. :)

| ⚠ | Operator ',' (Key Input) not implemented yet |
|---|---|

**Example usage**

```rust
mod brainfuck;
use crate::brainfuck::Interpreter;

fn main() {
    // initialized with only 256 bytes of memory - increase if more is needed
    let mut bf = Interpreter::new(256); 

    // Hello World
    bf.load_program("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");
    
    // clear console
    print!("\x1B[2J");

    // run and print state after program has finished
    // running (or stepping) will immediately print any "." operations
    bf.run();
    
    // prints the cpu and memory state
    bf.print_state();
}
```

