pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub status: u8,
    pub program_counter: u16,
    memory: [u8; 0xFFFF]
}

#[derive(Debug)]
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Implied,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Relative,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
    NoneAdressing,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            status: 0,
            program_counter: 0,
            memory: [0; 0xFFFF]
        }
    }

    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    fn mem_read_u16(&mut self, pos: u16) -> u16 {
        let lo = self.mem_read(pos) as u16;
        let hi = self.mem_read(pos + 1) as u16;
        (hi << 8) | (lo as u16)
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let lo = (data & 0xff) as u8;
        let hi = (data >> 8) as u8;
        self.mem_write(pos, lo);
        self.mem_write(pos + 1, hi)
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.status = 0;

        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000 .. (0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run()
    }

    // 0xA9
    fn lda(&mut self, value: u8) {
        self.register_a = value;
        self.update_zero_flag(self.register_a);
        self.update_negative_flag(self.register_a);
    }

    // 0xAA
    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_flag(self.register_x);
        self.update_negative_flag(self.register_x);
    }

    fn inx(&mut self) {
        self.register_x += 1;
        self.update_zero_flag(self.register_x);
        self.update_negative_flag(self.register_x);
    }

    fn update_zero_flag(&mut self, result: u8) {
        if result == 0 {
            self.status = self.status | 0b0000_0010;
        } else {
            self.status = self.status & 0b1111_1101;
        }
    }

    fn update_negative_flag(&mut self, result: u8) {
        if result & 0b1000_0000 != 0 {
            self.status = self.status | 0b1000_0000;
        } else {
            self.status = self.status & 0b0111_1111;
        }
    }

    pub fn run(&mut self) {
        loop {
            let opscode = self.mem_read(self.program_counter);
            self.program_counter += 1;

            match opscode {
                // LDA
                0xA9 => {
                    let param = self.mem_read(self.program_counter);
                    self.program_counter += 1;
                    self.lda(param);
                }
                // TAX
                0xAA => {
                    self.tax();
                }
                0xE8 => {
                    self.inx();
                }
                // BRK
                0x00 => {
                    return;
                }
                _ => todo!()
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // LDA (0XA9) TESTS
    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x45, 0x00]);
        assert_eq!(cpu.register_a, 0x45,
            "Register A should be holding 0x45, got 0x{:02x}", cpu.register_a);
        assert!(cpu.status & 0b0000_0010 == 0b0000_0000,
            "Zero flag should be 0, got 1");
        assert!(cpu.status & 0b1000_0000 == 0b0000_0000,
            "Negative flag should be 0, got 1");
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert_eq!(cpu.register_a, 0x00,
            "Register A should be holding 0x00, got 0x{:02x}", cpu.register_a);
        assert!(cpu.status & 0b0000_0010 == 0b0000_0010,
            "Zero flag should be 1, got 0");
        assert!(cpu.status & 0b1000_0000 == 0b0000_0000,
            "Negative flag should be 0, got 1");
    }

    #[test]
    fn test_0xa9_lda_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x80, 0x00]);
        assert_eq!(cpu.register_a, 0x80,
            "Register A should be holding 0x80, got 0x{:02x}", cpu.register_a);
        assert!(cpu.status & 0b0000_0010 == 0b0000_0000,
            "Zero flag should be 0, got 1");
        assert!(cpu.status & 0b1000_0000 == 0b1000_0000,
            "Negative flag should be 1, got 0");
    }

    #[test]
    fn test_0xa9_lda_zero_then_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0xa9, 0x80, 0x00]);
        assert_eq!(cpu.register_a, 0x80,
            "Register A should be holding 0x80, got 0x{:02x}", cpu.register_a);
        assert!(cpu.status & 0b0000_0010 == 0b0000_0000,
            "Zero flag should be 0, got 1");
        assert!(cpu.status & 0b1000_0000 == 0b1000_0000,
            "Negative flag should be 1, got 0");
    }

    // TAX (0XAA) TESTS
    #[test]
    fn test_0xaa_tax_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x45, 0xaa, 0x00]);
        assert_eq!(cpu.register_x, 0x45,
            "Register X should be holding 0x45, got 0x{:02x}", cpu.register_x);
        assert!(cpu.status & 0b0000_0010 == 0b0000_0000,
            "Zero flag should be 0, got 1");
        assert!(cpu.status & 0b1000_0000 == 0b0000_0000,
            "Negative flag should be 0, got 1");
    }

    #[test]
    fn test_0xaa_tax_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0xaa, 0x00]);
        assert_eq!(cpu.register_x, 0x00,
            "Register X should be holding 0x00, got 0x{:02x}", cpu.register_x);
        assert!(cpu.status & 0b0000_0010 == 0b0000_0010,
            "Zero flag should be 1, got 0");
        assert!(cpu.status & 0b1000_0000 == 0b0000_0000,
            "Negative flag should be 0, got 1");
    }

    #[test]
    fn test_0xaa_tax_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x80, 0xaa, 0x00]);
        assert_eq!(cpu.register_x, 0x80,
            "Register X should be holding 0x80, got 0x{:02x}", cpu.register_x);
        assert!(cpu.status & 0b0000_0010 == 0b0000_0000,
            "Zero flag should be 0, got 1");
        assert!(cpu.status & 0b1000_0000 == 0b1000_0000,
            "Negative flag should be 1, got 0");
    }

    // INX (0XE8) TESTS
    #[test]
    fn test_5_ops_working_together() {
       let mut cpu = CPU::new();
       cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
       assert_eq!(cpu.register_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0xaa, 0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1)
    }
}
