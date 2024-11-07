use crate::bus::*;
use crate::csr::*;
use crate::exception::*;
use crate::lib::address::*;
use crate::lib::cpu_instruction::*;

pub struct Cpu {
    pub regs: [u64; 32],
    pub pc: u64,
    pub csr: Csr,
    pub bus: Bus,
}

impl Cpu {
    pub fn new(binary: Vec<u8>) -> Self {
        let mut regs = [0; 32];
        regs[2] = (DRAM_BASE + DRAM_SIZE) as u64;
        Self {
            regs,
            pc: DRAM_BASE,
            bus: Bus::new(binary),
            csr: Csr::new(),
        }
    }
    /// Load a value from a dram.
    pub fn load(&mut self, addr: u64, size: u64) -> Result<u64, Exception> {
        self.bus.load(addr, size)
    }

    /// Store a value to a dram.
    pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), Exception> {
        self.bus.store(addr, size, value)
    }

    pub fn fetch(&self) -> Result<u64, Exception> {
        match self.bus.load(self.pc, 32) {
            Ok(inst) => Ok(inst),
            Err(e) => Err(e),
        }
    }

    pub fn execute(&mut self, inst: u64) -> Result<(), Exception> {
        let mut inst_step: usize = 4;
        let opcode = inst & 0b0111_1111;
        let rd = ((inst >> 7) & 0b1_1111) as usize;
        let funct3 = (inst >> 12) & 0b0111;
        let rs1 = ((inst >> 15) & 0b1_1111) as usize;
        let rs2 = ((inst >> 20) & 0b1_1111) as usize;
        let funct7 = (inst >> 25) & 0b111_1111;
        self.regs[0] = 0;

        return Err(Exception::IllegalInstruction(inst));
        self.pc = self.pc.wrapping_add(inst_step as u64);
        Ok(())
    }

    pub fn handle_exception(&self, exception: Exception) {}

    pub fn handle_interrupt(&self) {}
}
