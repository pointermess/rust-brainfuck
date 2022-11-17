mod brainfuck;
use crate::brainfuck::Interpreter;

fn main() {
    // initialized with only 256 bytes of memory - increase if more is needed
    let mut bf = Interpreter::new(256); 

    // Below are some test programs

    // Hello World
    bf.load_program("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");
    
    // fibonacci sequence - will run and calculate until stack overflow
    //bf.load_program(">++++++++++>+>+[[+++++[>++++++++<-]>.<++++++[>--------<-]+<<<]>.>>[[-]<[>+<-]>>[<<+>+>-]<[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>[-]>+>+<<<-[>+<-]]]]]]]]]]]+>>>]<<<]");
    
    // some simple testing code
    //bf.load_program("+[->++++[-->++++<]]");
    //bf.load_program("+>++>+++>++++---<--<->>>++++++++++");
    //bf.load_program("+++[>++<-]"); // fist byte = 0, second byte = 6
    //bf.load_program(">+++++++++[<++++++++>-]<."); // 0x48
    //bf.load_program("++++++++++++++++++++"); // just 20

    // clear console
    print!("\x1B[2J");

    // run and print state after program has finished
    // running (or stepping) will immediately
    bf.run();


    bf.print_state();
}
