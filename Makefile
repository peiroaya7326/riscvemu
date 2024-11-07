test.bin: test.c
	riscv64-unknown-elf-gcc -march=rv64g -nostdlib -Wl,-Ttext=0x0 -o test test.c
	riscv64-unknown-elf-objcopy -O binary test test.bin

objdump:
	riscv64-unknown-elf-objdump -D test

clean:
	rm -f test
	rm -f test.bin
