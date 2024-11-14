use crate::dram::*;
use crate::exception::*;
use crate::lib::address::*;
use crate::uart::*;

pub struct Bus {
    pub dram: Dram,
    pub uart: UART,
}

impl Bus {
    pub fn new(binary: Vec<u8>) -> Self {
        Self {
            dram: Dram::new(binary),
            uart: UART::new(),
        }
    }

    pub fn load(&self, addr: u64, size: u64) -> Result<u64, Exception> {
        if (UART_BASE <= addr) && (addr < UART_BASE + UART_SIZE) {
            return self.uart.load(addr);
        }
        if (DRAM_BASE <= addr) && (addr < DRAM_BASE + DRAM_SIZE) {
            return self.dram.load(addr, size);
        }
        Err(Exception::LoadAccessFault(addr))
    }
    pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), Exception> {
        if (UART_BASE <= addr) && (addr < UART_BASE + UART_SIZE) {
            return self.uart.store(addr, value);
        }

        if (DRAM_BASE <= addr) && (addr < DRAM_BASE + DRAM_SIZE) {
            return self.dram.store(addr, size, value);
        }
        Err(Exception::StoreAMOAccessFault(addr))
    }
}
