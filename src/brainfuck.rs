enum Operation {
    MoveLeft,
    MoveRight,
    Increment,
    Decrement,
    ConsolePrint,
    ConsoleRead,
    OpenLoop,
    CloseLoop,
    Skip
}
impl From<char> for Operation {
    fn from(command : char) -> Self {
        match command {
            '<' => Operation::MoveLeft,
            '>' => Operation::MoveRight,
            '+' => Operation::Increment,
            '-' => Operation::Decrement,
            '.' => Operation::ConsolePrint,
            ',' => Operation::ConsoleRead,
            '[' => Operation::OpenLoop,
            ']' => Operation::CloseLoop,
            _ => Operation::Skip
        }
    }
}

pub struct Interpreter {
    memory_pointer : usize,
    memory : Vec<u8>,

    program_counter : usize,
    program_code : Vec<char>,

    // stores program pointers of loop starts
    loops : Vec<usize>
}

impl Interpreter {
    pub fn new(memory_size : usize) -> Interpreter {
        Interpreter {
            memory_pointer: 0,
            memory: vec![0; memory_size],

            program_counter: 0,
            program_code: Vec::new(),

            loops: Vec::new(),
        }
    }

    // load a string into the memory vector
    pub fn load_program(&mut self, program : &str) {
        self.program_code = program.chars().collect();
    }

    // makes one step and returns if the end has been reached
    fn step(&mut self) -> bool {
        // immediately return with false if program has reached end
        if self.program_counter >= self.program_code.len() {
            return false;
        }

        // gets current character/operation and executes it.
        let ch = self.program_code[self.program_counter];
        match Operation::from(ch) {
            Operation::MoveLeft => self.move_left(),
            Operation::MoveRight => self.move_right(),
            Operation::Increment => self.increment(),
            Operation::Decrement => self.decrement(),
            Operation::ConsolePrint => self.print(),
            Operation::ConsoleRead => todo!(),
            Operation::OpenLoop => self.open_loop(),
            Operation::CloseLoop => self.close_loop(),
            Operation::Skip => {},
        }

        self.program_counter += 1;

        true
    }
    
    // moves the memory pointer to the left
    fn move_left(&mut self) {
        if self.memory_pointer > 0 {
            self.memory_pointer -= 1;
        } else {
            self.memory_pointer = self.memory.len() - 1;
        }
    }

    // moves memory pointer to the right
    fn move_right(&mut self) {
        if self.memory_pointer < self.memory.len() {
            self.memory_pointer += 1;
        } else {
            self.memory_pointer = 0;
        }
    }

    // decrements memory memory value
    fn decrement(&mut self) {
        self.memory[self.memory_pointer] = self.memory[self.memory_pointer].wrapping_sub(1);
    }

    // increments current memory value
    fn increment(&mut self) {
        self.memory[self.memory_pointer] = self.memory[self.memory_pointer].wrapping_add(1);
    }

    // runs until the end has been reached
    pub fn run(&mut self) {
        while self.step() {}
    }

    // prints character at current memory position
    fn print(&mut self) {
        print!("{}", char::from(self.memory[self.memory_pointer])); 
    }

    // handles open loop "["
    fn open_loop(&mut self) {
        // push new loop index
        if self.memory[self.memory_pointer] != 0 {
            self.loops.push(self.program_counter);
        } else {
            // skip loop
            let mut loop_counter = 0;
            
            // loop from current index to end to find where to skip to - breaks out of loop when found
            // todo: check if program is at end already ?
            for (index, ch) in self.program_code.iter().enumerate().skip(self.program_counter) {
                match ch {
                    '[' => loop_counter += 1,
                    ']' => loop_counter -= 1,
                    _ => {}
                };

                if loop_counter == 0 {
                    self.program_counter = index;
                    break;
                }
            }
        }
    }

    // handles close loop "]"
    fn close_loop(&mut self) {
        if self.memory[self.memory_pointer] != 0 {
            let item = self.loops[self.loops.len() - 1];
            self.program_counter = item;
        } else if !self.loops.is_empty() {
            self.loops.pop();
        }
    }

    // prints the cpu and memory state
    pub fn print_state(&self) {
        println!("\n---- CPU STATE ----");

        let code = self.program_code.iter().collect::<String>();

        println!("Program Code: {}", code);

        // display arrow at current program counter
        // todo: handle multiple lines for bigger programs
        let mut str : String = " ".repeat(self.program_counter);
        str.push('^');
        println!("              {}", str);


        println!("Program Counter: {}\n", self.program_counter);

        println!("---- MEM STATE ----");

        let mut c = 1;
        for byte in &self.memory {
            print!("{:02x} ", byte);

            if c % 16 == 0 {
                println!();
            }
            c += 1;
        }

        println!();
    }
}

#[cfg(test)]
mod tests {
    use crate::brainfuck::Interpreter;

    // prepares the interpreter with a test program and
    // returns the interpreter with an already executed program
    fn prepare_executed_program(program_code : &str) -> Interpreter {
        let mut interpreter = Interpreter::new(64);
        interpreter.load_program(program_code);
        interpreter.run();

        interpreter
    }

    #[test]
    fn addition() {
        let result = prepare_executed_program("++++++++").memory[0];
        assert_eq!(result, 8);
    }

    #[test]
    fn subtraction() {
        let result = prepare_executed_program("--------").memory[0];
        assert_eq!(result, 0xf8);
    }

    #[test]
    fn move_left() {
        let result = prepare_executed_program(">>>>").memory_pointer;
        assert_eq!(result, 4);
    }

    #[test]
    fn move_right() {
        let result = prepare_executed_program(">>>><<").memory_pointer;
        assert_eq!(result, 2);
    }

    #[test]
    fn looping() {
        let result = prepare_executed_program("+++[>+++++<-]").memory[1];
        assert_eq!(result, 15);
    }

    #[test]
    fn over_and_underflow() {
        let result = prepare_executed_program("+[+]-[-]").memory[0];
        assert_eq!(result, 0);
    }
}