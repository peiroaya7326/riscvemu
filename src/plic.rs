use crate::exception::*;
use crate::lib::address::*;
use crate::uart::UART_IRQ;

pub const INTERRUPT_PRIORITY: u64 = PLIC_BASE + 0x00_0000;
pub const INTERRUPT_PENDING: u64 = PLIC_BASE + 0x00_1000;
pub const INTERRUPT_ENABLES: u64 = PLIC_BASE + 0x00_2000;
pub const PRIORITY_THRESHOLD: u64 = PLIC_BASE + 0x20_0000;
pub const INTERRUPT_CLAIM: u64 = PLIC_BASE + 0x20_0004;
pub const INTERRUPT_COMPLETION: u64 = PLIC_BASE + 0x20_0004;

pub struct Plic {
    plic: [u8; PLIC_SIZE as usize],
    peripherals_irq: Vec<u64>,
}

impl Plic {
    pub fn new() -> Self {
        let plic = [0u8; PLIC_SIZE as usize];
        Self {
            plic,
            peripherals_irq: Vec::new(),
        }
    }

    pub fn add_irq(&mut self, irq: u64) {
        self.peripherals_irq.push(irq);
    }

    pub fn set_pending(&mut self, irq: u64) {
        let index = irq as usize / 8;
        let offset = irq as usize % 8;
        self.plic[(INTERRUPT_PENDING - PLIC_BASE) as usize + index] |= 1 << offset;
    }

    pub fn clear_pending(&mut self, irq: u64) {
        let index = irq as usize / 8;
        let offset = irq as usize % 8;
        self.plic[(INTERRUPT_PENDING - PLIC_BASE) as usize + index] &= !(1 << offset);
    }

    pub fn get_source_priority(&self, irq: u64) -> u64 {
        let irq_address = INTERRUPT_PRIORITY + irq * 4;
        return self.load(irq_address, 32).unwrap();
    }

    pub fn get_source_pending(&self, irq: u64) -> bool {
        let index = irq / 8;
        let offset = irq % 8;
        self.load(INTERRUPT_PENDING + index, 8).unwrap() >> offset & 0b1 != 0
    }

    pub fn get_source_enable(&self, irq: u64, hart: u64) -> bool {
        let enable_address = INTERRUPT_ENABLES + 0x80 * hart;
        let index = irq / 8;
        let offset = irq % 8;
        self.load(enable_address + index, 8).unwrap() >> offset & 0b1 != 0
    }

    pub fn get_hart_priority(&self, hart: u64) -> u64 {
        let hart_address = PRIORITY_THRESHOLD + hart * 0x1000;
        return self.load(hart_address, 32).unwrap();
    }

    pub fn check_pending(&self) -> Option<u64> {
        // TODO
        return None;
    }

    pub fn claim(&mut self) -> Option<u64> {
        // First, check if there are any pending interrupts
        if let Some(irq) = self.check_pending() {
            // Found a pending interrupt, now claim it
            // Clear the pending status of this interrupt, as it is now being processed
            let index = (irq / 8) as usize;
            let bit = (irq % 8) as u8;
            self.plic[(INTERRUPT_PENDING - PLIC_BASE) as usize + index] &= !(1 << bit);
            self.store(INTERRUPT_CLAIM + 0x1000, 32, irq).unwrap();
            return Some(irq);
        }
        // If no interrupts are pending, return None
        None
    }

    pub fn completion(&mut self, irq: u64) {}

    pub fn load(&self, addr: u64, size: u64) -> Result<u64, Exception> {
        if size % 8 != 0 {
            return Err(Exception::LoadAccessFault(addr));
        }
        let index = (addr - PLIC_BASE) as usize;
        let mut value = self.plic[index] as u64;
        for i in 1..size / 8 {
            value |= (self.plic[index + i as usize] as u64) << (i * 8);
        }
        Ok(value)
    }

    pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), Exception> {
        if size % 8 != 0 {
            return Err(Exception::StoreAMOAccessFault(addr));
        }
        let index = (addr - PLIC_BASE) as usize;
        for i in 0..size / 8 {
            self.plic[index + i as usize] = (value >> (i * 8) & 0xff) as u8;
        }
        Ok(())
    }
}
