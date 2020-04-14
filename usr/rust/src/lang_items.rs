use crate::syscall::sys_exit;
// use core::alloc::Layout;
use core::panic::PanicInfo;

#[linkage = "weak"]
#[no_mangle]
fn main() -> usize {
    panic!("No main() linked");
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let location = _info.location().unwrap();
    let message = _info.message().unwrap();
    println!(
        "\nPANIC in {} at line {} \n\t{}",
        location.file(),
        location.line(),
        message
    );
    loop {}
}

#[no_mangle]
pub extern "C" fn _start(_args: isize, _argv: *const u8) -> ! {
    // init_heap();
    sys_exit(main())
}

#[no_mangle]
pub extern "C" fn abort() {
    panic!("abort");
}

#[lang = "oom"]
fn oom(_: Layout) -> ! {
    panic!("out of memory!");
}
