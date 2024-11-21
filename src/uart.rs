use crate::exception::*;
use crate::lib::address::*;
use crate::plic::*;
use emu_nb_stdin::EmuNbStdin;
use std::cell::RefCell;
use std::io::{self, Write};
use std::rc::Rc;

pub const UART_IRQ: u64 = 10;

pub const RHR: u64 = UART_BASE + 0b000;
pub const THR: u64 = UART_BASE + 0b000;
pub const _IER: u64 = UART_BASE + 0b001;
pub const _ISR: u64 = UART_BASE + 0b010;
pub const _FCR: u64 = UART_BASE + 0b010;
pub const _LCR: u64 = UART_BASE + 0b011;
pub const _MCR: u64 = UART_BASE + 0b100;
pub const LSR: u64 = UART_BASE + 0b101;
pub const _MSR: u64 = UART_BASE + 0b110;
pub const _SPR: u64 = UART_BASE + 0b111;

pub const LSR_DATA_READY: u8 = 1;
pub const LSR_THR_EMPTY: u8 = 1 << 5;

pub struct UART {
    uart: Vec<u8>,
    in_fd: EmuNbStdin,
    plic: Rc<RefCell<Plic>>,
}

impl UART {
    pub fn new(plic: Rc<RefCell<Plic>>) -> Self {
        let mut uart = vec![0u8; UART_SIZE as usize];
        uart[(LSR - UART_BASE) as usize] |= LSR_THR_EMPTY;
        plic.borrow_mut().add_irq(UART_IRQ);
        Self {
            uart,
            in_fd: EmuNbStdin::new(),
            plic,
        }
    }

    pub fn check_interrupt(&mut self) {
        if self.in_fd.poll() {
            self.uart[(LSR - UART_BASE) as usize] |= LSR_DATA_READY;
            self.plic.borrow_mut().set_pending(UART_IRQ);
        }
    }

    pub fn load(&mut self, addr: u64, size: u64) -> Result<u64, Exception> {
        if size != 8 {
            return Err(Exception::LoadAccessFault(addr));
        }
        match addr {
            RHR => {
                self.uart[(LSR - UART_BASE) as usize] &= !LSR_DATA_READY;
                if let Some(value) = self.in_fd.receive() {
                    Ok(value as u64)
                } else {
                    Err(Exception::LoadAccessFault(addr))
                }
            }
            _ => Ok(self.uart[(addr - RHR) as usize] as u64),
        }
    }

    pub fn store(&mut self, addr: u64, value: u64) -> Result<(), Exception> {
        match addr {
            THR => {
                print!("{}", value as u8 as char);
                io::stdout()
                    .flush()
                    .expect("Failed to flush stdout after writing to UART");
            }
            _ => {
                self.uart[(addr - UART_BASE) as usize] = (value & 0xff) as u8;
            }
        }
        Ok(())
    }
}
