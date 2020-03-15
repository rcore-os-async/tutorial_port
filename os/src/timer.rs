use crate::sbi::set_timer;
use riscv::register::{sie, time};

pub static mut TICKS: usize = 0;

static TIMEBASE: u64 = 100000;
pub fn init() {
    unsafe {
        TICKS = 0;
        sie::set_stimer();
    }
    clock_set_next_event();
    println!("++++ setup timer!     ++++");
}

pub fn clock_set_next_event() {
    set_timer(get_cycle() + TIMEBASE);
}

pub fn get_cycle() -> u64 {
    time::read() as u64
}

pub fn get_time() -> u64 {
    get_cycle() / TIMEBASE
}
