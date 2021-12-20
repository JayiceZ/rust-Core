#![no_std]
#![no_main]
#![feature(llvm_asm)]


use core::fmt;
use core::fmt::Write;


mod lang_items;

//sys call
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;

//sbi call
const SBI_SHUTDOWN: usize = 8;



fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize = 0;
    unsafe {
        llvm_asm!("ecall"
            : "={x10}" (ret)
            : "{x10}" (args[0]), "{x11}" (args[1]), "{x12}" (args[2]), "{x17}" (id)
            : "memory"
            : "volatile"
        );
    }
    ret
}

fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret = 0;
    unsafe {
        llvm_asm!("ecall"
            : "={x10}" (ret)
            : "{x10}" (arg0), "{x11}" (arg1), "{x12}" (arg2), "{x17}" (which)
        );
    }
    ret
}





pub fn sys_exit(xstate: i32) -> isize {
    syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
}

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

pub fn shut_down()->!{
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);
    panic!("should not reach here");
}





// compiler will see it as entrance
#[no_mangle]
extern "C" fn _start() {
    //println!("Hello, world!");   it base on linux kernel
    shut_down();
}


struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        sys_write(1, s.as_bytes());
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}


#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
