An operating system kernel running on RISC-V arch

## How to build it
### Environmental requirement
- rustup ( >=1.57.0-nightly)
- Qemu ( >=5.0.0)

### Step

##### Build locally

```
cd os
cargo build --release
```
then we get ELF file in `target/riscv64gc-unknown-none-elf/release`

then we should transfer ELF file into binary file by executing
```
rust-objcopy --binary-architecture=riscv64 target/riscv64gc-unknown-none-elf/release/os --strip-all -O binary target/riscv64gc-unknown-none-elf/release/os.bin
```

finally we can run our kernel on QEMU
```
qemu-system-riscv64 -machine virt -nographic -bios ../bootloader/rustsbi-qemu.bin -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000
```