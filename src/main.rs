mod bus;
mod cpu;
mod csr;
mod dram;
mod exception;
mod interrupt;
mod lib;

use cpu::*;
use exception::*;
use interrupt::*;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: rrvemu <filename>");
    }
    let mut file = File::open(&args[1])?;
    let mut binary = Vec::new();
    file.read_to_end(&mut binary)?;

    let mut cpu = Cpu::new(binary);

    loop {
        let inst = match cpu.fetch() {
            Ok(inst) => inst,
            Err(e) => {
                cpu.handle_exception(e);
                if e.is_fatal() {
                    println!("Failed to fetch instruction: {:?}", e);
                    break;
                }
                continue;
            }
        };
        match cpu.execute(inst) {
            Ok(_) => {}
            Err(e) => {
                if e.is_fatal() {
                    println!("Failed to execute instruction: {:?}", e);
                    break;
                } else {
                    cpu.handle_exception(e);
                }
            }
        }
        match cpu.check_interrupt() {
            Some(interrupt) => cpu.handle_interrupt(interrupt),
            None => (),
        }
    }
    cpu.print_registers();
    Ok(())
}
