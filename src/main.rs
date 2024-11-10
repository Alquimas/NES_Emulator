pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub status: u8,
    pub program_counter: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            status: 0,
            program_counter: 0,
        }
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

    pub fn interpret(&mut self, program: Vec<u8>) {
        self.program_counter = 0;

        loop {
            let opscode = program[self.program_counter as usize];
            self.program_counter += 1;

            match opscode {
                // LDA
                0xA9 => {
                    let param = program[self.program_counter as usize];
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

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    // LDA (0XA9) TESTS
    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x45, 0x00]);
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
        cpu.interpret(vec![0xa9, 0x00, 0x00]);
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
        cpu.interpret(vec![0xa9, 0x80, 0x00]);
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
        cpu.interpret(vec![0xa9, 0x00, 0xa9, 0x80, 0x00]);
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
        cpu.register_a = 0x45;
        cpu.interpret(vec![0xaa, 0x00]);
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
        cpu.register_a = 0x00;
        cpu.interpret(vec![0xaa, 0x00]);
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
        cpu.register_a = 0x80;
        cpu.interpret(vec![0xaa, 0x00]);
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
       cpu.interpret(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
       assert_eq!(cpu.register_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;
        cpu.interpret(vec![0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1)
    }
}
