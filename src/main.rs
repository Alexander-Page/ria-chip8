use core::panic;

/*
Op codes:
ADD 0x8014,
CALL 0x2nnn, (nnn is a mem address)
RETURN 0x00EE,
*/
enum CpuOpperation {
    Add(u8, u8),
    Call(u16),
    Ret,
    Halt,
    DNE,
}

// last register reserved for carry flag
struct CPU {
    memory: [u8; 0x1000],
    registers: [u8; 16],
    program_counter: usize,
    stack: [u16; 16],
    stack_pointer: usize,
}

impl CPU {
    fn new() -> Self {
        CPU {
            memory: [0; 0x1000],
            registers: [0; 16],
            program_counter: 0,
            stack: [0; 16],
            stack_pointer: 0,
        }
    }

    fn read_opcode(&self) -> u16 {
        let part1 = self.memory[self.program_counter] as u16;
        let part2 = self.memory[self.program_counter + 1] as u16;

        // combine part 1 and part 2 into one u16 instruction
        (part1 << 8) | part2
    }

    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("Stack Overflow");
        }

        stack[sp] = self.program_counter as u16;
        self.stack_pointer += 1;
        self.program_counter = addr as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 as usize {
            panic!("Stack Underflow");
        }
        self.stack_pointer -= 1;
        self.program_counter = self.stack[self.stack_pointer] as usize;
    }

    fn run(&mut self) {
        loop {
            let decoded = self.decode_opcode();
            self.program_counter += 2;

            match decoded {
                CpuOpperation::Halt => {
                    return;
                }
                CpuOpperation::Add(i, j) => self.add_xy(i as u8, j as u8),
                CpuOpperation::Call(addr) => self.call(addr),
                CpuOpperation::Ret => self.ret(),
                _ => todo!(),
            }
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    fn decode_opcode(&self) -> CpuOpperation {
        // get the values in the instruction
        let instruction = self.read_opcode();
        let c = ((instruction & 0xf000) >> 12) as u8;
        let x = ((instruction & 0x0f00) >> 8) as u8;
        let y = ((instruction & 0x00f0) >> 4) as u8;
        let d = ((instruction & 0x000f) >> 0) as u8;

        match (c, x, y, d) {
            (0, 0, 0, 0) => CpuOpperation::Halt,
            (0x8, _, _, 0x4) => CpuOpperation::Add(x, y),
            (0x2, _, _, _) => CpuOpperation::Call(instruction & 0x0FFF),
            (0, 0, 0xE, 0xE) => CpuOpperation::Ret,
            _ => CpuOpperation::DNE,
        }
    }
}
fn main() {
    let mut cpu = CPU::new();
    cpu.registers[0] = 3;
    cpu.registers[1] = 6;

    cpu.memory[0x000] = 0x21;
    cpu.memory[0x001] = 0x00;
    cpu.memory[0x002] = 0x21;
    cpu.memory[0x003] = 0x00;
    cpu.memory[0x004] = 0x00;
    cpu.memory[0x005] = 0x00;

    cpu.memory[0x100] = 0x80;
    cpu.memory[0x101] = 0x14;
    cpu.memory[0x102] = 0x80;
    cpu.memory[0x103] = 0x14;
    cpu.memory[0x104] = 0x00;
    cpu.memory[0x105] = 0xEE;

    println!(
        "reg 0 = {} \t reg 1 = {}",
        cpu.registers[0], cpu.registers[1]
    );

    cpu.run();

    println!(
        "reg 0 = {} \t reg 1 = {}",
        cpu.registers[0], cpu.registers[1]
    );
}
