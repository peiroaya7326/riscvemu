use crate::clint::Clint;
use crate::dram::*;
use crate::exception::*;
use crate::lib::address::*;
use crate::plic::Plic;
use crate::uart::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Bus {
    pub dram: Dram,
    pub plic: Rc<RefCell<Plic>>,
    pub uart: UART,
    pub clint: Clint,
}

impl Bus {
    pub fn new(timer_freq: u64, binary: Vec<u8>) -> Self {
        let plic = Rc::new(RefCell::new(Plic::new()));
        Self {
            dram: Dram::new(binary),
            uart: UART::new(Rc::clone(&plic)),
            plic,
            clint: Clint::new(timer_freq),
        }
    }

    pub fn load(&mut self, addr: u64, size: u64) -> Result<u64, Exception> {
        if (CLINT_BASE <= addr) && (addr < CLINT_BASE + CLINT_SIZE) {
            return self.clint.load(addr);
        }
        if (PLIC_BASE <= addr) && (addr < PLIC_BASE + PLIC_SIZE) {
            return self.plic.borrow_mut().load(addr, size);
        }
        if (UART_BASE <= addr) && (addr < UART_BASE + UART_SIZE) {
            return self.uart.load(addr, size);
        }
        if (DRAM_BASE <= addr) && (addr < DRAM_BASE + DRAM_SIZE) {
            return self.dram.load(addr, size);
        }
        Err(Exception::LoadAccessFault(addr))
    }
    pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), Exception> {
        if (CLINT_BASE <= addr) && (addr < CLINT_BASE + CLINT_SIZE) {
            return self.clint.store(addr, value);
        }
        if (PLIC_BASE <= addr) && (addr < PLIC_BASE + PLIC_SIZE) {
            return self.plic.borrow_mut().store(addr, size, value);
        }
        if (UART_BASE <= addr) && (addr < UART_BASE + UART_SIZE) {
            return self.uart.store(addr, value);
        }
        if (DRAM_BASE <= addr) && (addr < DRAM_BASE + DRAM_SIZE) {
            return self.dram.store(addr, size, value);
        }
        Err(Exception::StoreAMOAccessFault(addr))
    }
}
