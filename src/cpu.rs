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
        let op1 = inst & 0b11;
        let op2 = (inst >> 2) & 0b111;
        let op3 = (inst >> 5) & 0b11;
        let rd = ((inst >> 7) & 0x1f) as usize;
        let rs1 = ((inst >> 15) & 0x1f) as usize;
        let rs2 = ((inst >> 20) & 0x1f) as usize;
        let funct3 = (inst >> 12) & 0x7;
        let funct7 = (inst >> 25) & 0x7f;

        let mut inst_step: usize = 4;
        if op1 == 0b10 {
            inst_step = 2;
        } else if op1 == 0b11 {
            inst_step = 4;
        } else {
            return Err(Exception::IllegalInstruction(inst));
        }

        self.regs[0] = 0;

        match (op3, op2) {
            // Group 0 (inst[6:5] == 00)
            (0b00, 0b000) => {
                // Load
                match funct3 {
                    0b000 => self.execute_lb(),
                    0b001 => self.execute_lh(),
                    0b010 => self.execute_lw(),
                    0b100 => self.execute_lbu(),
                    0b101 => self.execute_lhu(),
                    0b110 => self.execute_lwu(),
                    0b011 => self.execute_ld(),
                    _ => {
                        return Err(Exception::IllegalInstruction(inst));
                    }
                }
            }
            (0b00, 0b001) => {
                // LOAD-FP
                return Err(Exception::IllegalInstruction(inst));
            }
            (0b00, 0b010) => {
                // custom-0
                return Err(Exception::IllegalInstruction(inst));
            }
            (0b00, 0b011) => {
                // MISC-MEM
                // FENCE
                return Err(Exception::IllegalInstruction(inst));
            }
            (0b00, 0b100) => {
                // OP-IMM
                match (funct7, funct3) {
                    (_, 0b000) => self.execute_addi(),
                    (_, 0b010) => self.execute_slti(),
                    (_, 0b011) => self.execute_sltiu(),
                    (_, 0b100) => self.execute_xori(),
                    (_, 0b110) => self.execute_ori(),
                    (_, 0b111) => self.execute_andi(),
                    (0b000_0000, 0b001) => self.execute_slli(),
                    (0b000_0000, 0b101) => self.execute_srli(),
                    (0b010_0000, 0b101) => self.execute_srai(),
                    _ => {
                        return Err(Exception::IllegalInstruction(inst));
                    }
                }
            }
            (0b00, 0b101) => {
                // AUIPC
                self.execute_auipc()
            }
            (0b00, 0b110) => {
                // OP-IMM32
                match (funct7, funct3) {
                    (_, 0b000) => self.execute_addi(),
                    (0b000_0000, 0b001) => self.execute_slti(),
                    (0b000_0000, 0b101) => self.execute_sltiu(),
                    (0b010_0000, 0b101) => self.execute_xori(),
                    _ => {
                        return Err(Exception::IllegalInstruction(inst));
                    }
                }
            }

            // Group 1 (inst[6:5] == 01)
            (0b01, 0b000) => {
                // Store
                match funct3 {
                    0b000 => self.execute_sb(),
                    0b001 => self.execute_sh(),
                    0b010 => self.execute_sw(),
                    0b011 => self.execute_sd(),
                    _ => {
                        return Err(Exception::IllegalInstruction(inst));
                    }
                }
            }
            (0b01, 0b001) => {
                // STORE-FP
                return Err(Exception::IllegalInstruction(inst));
            }
            (0b01, 0b010) => {
                // custom-1
                return Err(Exception::IllegalInstruction(inst));
            }
            (0b01, 0b011) => {
                // AMO
                return Err(Exception::IllegalInstruction(inst));
            }
            (0b01, 0b100) => {
                // OP
                match (funct7, funct3) {
                    (0b000_0000, 0b000) => self.execute_add(),
                    (0b010_0000, 0b000) => self.execute_sub(),
                    (0b000_0000, 0b001) => self.execute_sll(),
                    (0b000_0000, 0b010) => self.execute_slt(),
                    (0b000_0000, 0b011) => self.execute_sltu(),
                    (0b000_0000, 0b100) => self.execute_xor(),
                    (0b000_0000, 0b101) => self.execute_srl(),
                    (0b010_0000, 0b101) => self.execute_sra(),
                    (0b000_0000, 0b110) => self.execute_or(),
                    (0b000_0000, 0b111) => self.execute_and(),
                    (0b000_0001, 0b000) => self.execute_mul(),
                    (0b000_0001, 0b001) => self.execute_mulh(),
                    (0b000_0001, 0b010) => self.execute_mulhsu(),
                    (0b000_0001, 0b011) => self.execute_mulhu(),
                    (0b000_0001, 0b100) => self.execute_div(),
                    (0b000_0001, 0b101) => self.execute_divu(),
                    (0b000_0001, 0b110) => self.execute_rem(),
                    (0b000_0001, 0b111) => self.execute_remu(),
                    _ => {
                        return Err(Exception::IllegalInstruction(inst));
                    }
                }
            }
            (0b01, 0b101) => {
                // LUI
                self.execute_lui()
            }
            (0b01, 0b110) => {
                // OP-32
                match (funct7, funct3) {
                    (0b000_0000, 0b000) => self.execute_addw(),
                    (0b010_0000, 0b000) => self.execute_subw(),
                    (0b000_0000, 0b001) => self.execute_sllw(),
                    (0b000_0000, 0b101) => self.execute_srlw(),
                    (0b010_0000, 0b101) => self.execute_sraw(),
                    (0b000_0001, 0b000) => self.execute_mulw(),
                    (0b000_0001, 0b100) => self.execute_divw(),
                    (0b000_0001, 0b101) => self.execute_divuw(),
                    (0b000_0001, 0b110) => self.execute_remw(),
                    (0b000_0001, 0b111) => self.execute_remuw(),
                    _ => {
                        return Err(Exception::IllegalInstruction(inst));
                    }
                }
            }

            // Group 2 (inst[6:5] == 10)
            (0b10, 0b000) => {
                // MADD
                return Err(Exception::IllegalInstruction(inst));
            }
            (0b10, 0b001) => {
                // MSUB
                return Err(Exception::IllegalInstruction(inst));
            }
            (0b10, 0b010) => {
                // NMSUB
                return Err(Exception::IllegalInstruction(inst));
            }
            (0b10, 0b011) => {
                // NMADD
                return Err(Exception::IllegalInstruction(inst));
            }
            (0b10, 0b100) => {
                // OP-FP
                return Err(Exception::IllegalInstruction(inst));
            }
            (0b10, 0b110) => {
                // custom-2/rv128
                return Err(Exception::IllegalInstruction(inst));
            }

            // Group 3 (inst[6:5] == 11)
            (0b11, 0b000) => {
                // BRANCH
                match funct3 {
                    0b000 => self.execute_beq(),
                    0b001 => self.execute_bne(),
                    0b100 => self.execute_blt(),
                    0b101 => self.execute_bge(),
                    0b110 => self.execute_bltu(),
                    0b111 => self.execute_bgeu(),
                    _ => {
                        return Err(Exception::IllegalInstruction(inst));
                    }
                }
            }
            (0b11, 0b001) => {
                // JALR
                self.execute_jalr()
            }
            (0b11, 0b010) => {
                // reserved
                return Err(Exception::IllegalInstruction(inst));
            }
            (0b11, 0b011) => {
                // JAL
                self.execute_jal()
            }
            (0b11, 0b100) => {
                // SYSTEM
                match (rs2, funct3) {
                    (_, 0b001) => self.execute_csrrw(),
                    (_, 0b010) => self.execute_csrrs(),
                    (_, 0b011) => self.execute_csrrc(),
                    (_, 0b101) => self.execute_csrrwi(),
                    (_, 0b110) => self.execute_csrrsi(),
                    (_, 0b111) => self.execute_csrrci(),
                    (0b00000, 0b000) => self.execute_divw(),
                    (0b00001, 0b000) => self.execute_divuw(),
                    // (0b01101, 0b000) => self.execute_wrs_nto(),
                    // (0b11101, 0b000) => self.execute_wrs_sto(),
                    _ => {
                        return Err(Exception::IllegalInstruction(inst));
                    }
                }
            }
            (0b11, 0b101) => {
                // OP-VE
                return Err(Exception::IllegalInstruction(inst));
            }
            (0b11, 0b110) => {
                // custom-3/rv128
                return Err(Exception::IllegalInstruction(inst));
            }

            // Default case (any other combination)
            _ => {
                return Err(Exception::IllegalInstruction(inst));
            }
        }

        self.pc = self.pc.wrapping_add(inst_step as u64);
        Ok(())
    }

    pub fn handle_exception(&self, exception: Exception) {}

    pub fn handle_interrupt(&self) {}
}
