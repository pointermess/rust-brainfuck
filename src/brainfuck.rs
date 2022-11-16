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

impl Operation {
    fn from_char(command : &char) -> Operation {
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

// Memory

pub trait HandleMemory {
    fn move_left(&mut self);
    fn move_right(&mut self);

    fn decrement(&mut self);
    fn increment(&mut self);

    fn read(&self) -> u8;
}

pub struct Memory {
    memory_pointer : usize,
    memory : Vec<u8>,
}

impl Memory {
    pub fn new(memory_size : i32) -> Memory {
        let mut vector : Vec<i32> = Vec::new();

        for _i in [0..memory_size] {
            vector.push(0);
        }

        Memory { 
            memory_pointer: 0,
            memory: vec![0, 255],
        }
    }
}

// todo check bounds
impl HandleMemory for Memory {
    fn move_left(&mut self) {
        if self.memory_pointer > 0 {
            self.memory_pointer -= 1;
        }
    }

    fn move_right(&mut self) {
        self.memory_pointer += 1;
    }

    fn decrement(&mut self) {
        self.memory[self.memory_pointer] -= 1;
    }

    fn increment(&mut self) {
        self.memory[self.memory_pointer] += 1;
    }

    fn read(&self) -> u8 {
        return self.memory[self.memory_pointer];
    }
}

pub trait HandleProgram {
    fn load_program(&mut self, program : &str);

    fn step(&mut self) -> bool;

    fn run(&mut self);

    fn print_state(&self);
}

pub struct Interpreter {
    memory : Memory,

    program_counter : usize,
    program_code : Vec<char>
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            memory: Memory::new(4096),

            program_counter: 0,
            program_code: Vec::new(),
        }
    }
    
    pub fn get_memory(&mut self) -> &mut Memory {
        return &mut self.memory;
    }


}

impl HandleProgram for Interpreter {
    fn load_program(&mut self, program : &str) {
        self.program_code = program.chars().collect();
    }

    fn step(&mut self) -> bool {
        if self.program_counter >= self.program_code.len() {
            return false;
        }

        let ch = self.program_code[self.program_counter];

        println!("Running operation: {}", ch);

        // todo implement operations
        match Operation::from_char(&ch) {
            Operation::MoveLeft => self.memory.move_left(),
            Operation::MoveRight => self.memory.move_right(),
            Operation::Increment => self.memory.increment(),
            Operation::Decrement => self.memory.decrement(),
            Operation::ConsolePrint => {},
            Operation::ConsoleRead => {},
            Operation::OpenLoop => {},
            Operation::CloseLoop => {},
            Operation::Skip => {},
        }

        self.program_counter += 1;

        return true;
    }

    fn run(&mut self) {
        if self.step() {
            self.run();
        }
    }

    fn print_state(&self) {
        println!("---- STATE ----");

        let code = self.program_code.iter().cloned().collect::<String>();

        println!("Program Code: {}", code);
        println!("Program Counter: {}", self.program_counter);
    }
}