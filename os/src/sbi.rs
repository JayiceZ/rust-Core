#![allow(unused)]

//sbi call
const SBI_SHUTDOWN: usize = 8;
const SBI_CONSOLE_PUTCHAR: usize = 1;


#[inline(always)]
fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret ;
    unsafe {
        asm!("ecall",
            inlateout("x10") arg0 => ret,
            in("x11") arg1,
            in("x12") arg2,
            in("x17") which,
        );
    }
    ret
}



pub fn shut_down()->!{
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);
    panic!("should not reach here");
}

pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, c,0,0,);
}