#![no_std]
#![no_main]


mod lang_items;

// compiler will see it as entrance
extern "C" fn _start() {
    loop{};
}