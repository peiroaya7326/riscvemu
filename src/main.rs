mod bus;
mod cpu;
mod csr;
mod dram;
mod exception;
mod lib;

use cpu::*;
use exception::*;
use lib::cpu_inspect::*;
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
                    println!("{:?}", e);
                    break;
                }
                continue;
            }
        };
        match cpu.execute(inst) {
            Ok(_) => {}
            Err(e) => {
                if e.is_fatal() {
                    println!("{:x?}", e);
                    break;
                }
            }
        }
    }
    cpu.print_registers();
    Ok(())
}
