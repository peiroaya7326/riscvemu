test.bin: test.c
	riscv64-unknown-elf-gcc -march=rv64g -mcmodel=medany -nostdlib -T test.ld -o test test.c
	riscv64-unknown-elf-objcopy -O binary test test.bin

objdump:
	riscv64-unknown-elf-objdump -d test

clean:
	rm -f test
	rm -f test.bin
