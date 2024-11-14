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
        let mut plic = [0u8; PLIC_SIZE as usize];
        Self {
            plic,
            peripherals_irq: Vec::new(),
        }
    }

    pub fn set_pending(&mut self, irq: u64) {
        let index = irq as usize / 8;
        let offset = irq as usize % 8;
        self.plic[(INTERRUPT_PENDING - PLIC_BASE) as usize + index] = 1 << offset;
    }

    pub fn check_pending(&self) -> Option<u64> {
        let mut highest_priority_irq = None;
        let mut highest_priority = 0;

        // Considering hart1 (supervisor mode), retrieving its threshold register value
        let hart1_threshold_addr = PRIORITY_THRESHOLD + 0x1000; // Address for hart1's threshold register
        let hart_threshold = self.load(hart1_threshold_addr - PLIC_BASE, 32).unwrap();

        // Iterate through all the registered peripheral IRQs to find pending interrupts
        for &irq in &self.peripherals_irq {
            let index = irq as usize / 8;
            let offset = irq as usize % 8;

            // Check if the IRQ is in a pending state
            let is_pending =
                (self.plic[(INTERRUPT_PENDING - PLIC_BASE) as usize + index] & (1 << offset)) != 0;

            // Compute the address for hart1's enable register and check if the IRQ is enabled
            let hart1_enable_addr = INTERRUPT_ENABLES + 0x80; // Offset 0x80 corresponds to hart1 (supervisor mode) enable base
            let is_enabled =
                (self.plic[(hart1_enable_addr - PLIC_BASE) as usize + index] & (1 << offset)) != 0;

            // If the interrupt is not pending or not enabled, skip it
            if !(is_pending && is_enabled) {
                continue;
            }

            // Check the priority of the interrupt
            let priority_addr = INTERRUPT_PRIORITY + irq * 4;
            let priority = self.load(priority_addr - PLIC_BASE, 32).unwrap();

            // If the interrupt priority is greater than the hart's threshold and higher than the current highest priority, update the result
            if priority > hart_threshold && priority > highest_priority {
                highest_priority = priority;
                highest_priority_irq = Some(irq);
            }
        }

        highest_priority_irq
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
