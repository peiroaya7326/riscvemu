use crate::exception::*;
use crate::lib::address::*;
use std::io::Read;
use std::io::{self, Write};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

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
    uart: Arc<(Mutex<[u8; UART_SIZE as usize]>, Condvar)>,
}

impl UART {
    pub fn new() -> Self {
        let uart = Arc::new((Mutex::new([0u8; UART_SIZE as usize]), Condvar::new()));
        let uart_for_recieve = Arc::clone(&uart);
        let recieve_uart_loop = thread::spawn(move || Self::recieve_uart_loop(uart_for_recieve));
        Self { uart }
    }

    pub fn recieve_uart_loop(uart_for_read: Arc<(Mutex<[u8; UART_SIZE as usize]>, Condvar)>) {
        let mut input_buffer = [0u8; 1];

        loop {
            match io::stdin().read(&mut input_buffer) {
                Ok(_) => {
                    let (uart_buffer, condvar) = &*uart_for_read;
                    let mut uart_buffer = uart_buffer
                        .lock()
                        .expect("Failed to lock UART buffer for recieving");

                    // Wait if Transmitter Holding Register (THR) is empty
                    while uart_buffer[(LSR - UART_BASE) as usize] & LSR_DATA_READY == 1 {
                        uart_buffer = condvar
                            .wait(uart_buffer)
                            .expect("Failed to wait on UART condition variable");
                    }
                    uart_buffer[0] = input_buffer[0];
                    println!("Success {}", input_buffer[0]);
                }
                Err(e) => {
                    eprintln!("Error reading UART input: {:?}", e);
                }
            }
        }
    }

    pub fn load(&self, addr: u64) -> Result<u64, Exception> {
        // match addr {}
        let (uart_buffer, cvar) = &*self.uart;
        let mut uart_buffer = uart_buffer
            .lock()
            .expect("Failed to lock UART buffer for reading");

        match addr {
            RHR => {
                cvar.notify_one();
                uart_buffer[(addr - UART_BASE) as usize] &= !LSR_DATA_READY;
            }
            _ => {}
        }
        Ok(uart_buffer[(addr - UART_BASE) as usize] as u64)
    }

    pub fn store(&mut self, addr: u64, value: u64) -> Result<(), Exception> {
        let (uart_buffer, cvar) = &*self.uart;
        let mut uart_buffer = uart_buffer
            .lock()
            .expect("Failed to lock UART buffer for writing");
        match addr {
            THR => {
                print!("{}", (value & 0xff) as u8 as char);
                io::stdout()
                    .flush()
                    .expect("Failed to flush stdout after writing to UART");
            }
            _ => {
                uart_buffer[(addr - UART_BASE) as usize] = (value & 0xff) as u8;
            }
        }
        Ok(())
    }
}
