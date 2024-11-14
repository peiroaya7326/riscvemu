use crate::exception::*;
use crate::lib::address::*;

pub struct Plic {
    plic: [u8; PLIC_SIZE as usize],
}

impl Plic {
    pub fn new() -> Self {
        let mut plic = [0u8; PLIC_SIZE as usize];
        Self { plic }
    }

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
