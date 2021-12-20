#![feature(panic_info_message)]
use crate::sbi::shut_down;
use core::panic::PanicInfo;


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!("Panicked at {}:{} {}", location.file(), location.line(), info.message().unwrap());
    } else {
        println!("Panicked: {}", info.message().unwrap());
    }
    shut_down()
}