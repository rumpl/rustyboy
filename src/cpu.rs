use crate::mbc::MBC;
use crate::register::Registers;

#[derive(Debug)]
pub struct Cpu {
    pub registers: Registers,
    pub mbc: MBC,
}

impl Cpu {
    pub fn new(mbc: MBC) -> Cpu {
        Cpu {
            registers: Registers::new(),
            mbc: mbc,
        }
    }

    fn word(&mut self) -> u16 {
        let w = self.mbc.read(self.registers.pc);
        self.registers.pc += 2;
        w as u16
    }

    pub fn cycle(&mut self) {
        let opcode = self.mbc.read(self.registers.pc);
        self.registers.pc += 1;

        let res = match opcode {
            0x00 => 1,
            0x21 => 1,
            0x31 => {
                self.registers.sp = self.word();
                12
            }
            0xAF => {
                self.alu_xor(self.registers.a);
                4
            }
            // 0xc3 => {self.registers.pc = self.word(), 2}
            other => panic!("Instruction {:2X} not implemented", other),
        };

        println!("{}", res);
    }

    fn alu_xor(&mut self, b: u8) {
        let r = self.registers.a ^ b;
        self.registers.a = r;
        // self.registers.
    }
}
