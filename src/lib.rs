#![no_std]

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    extern "C" {
        fn panic_nonexistent() -> !;
    }
    unsafe { panic_nonexistent() }
}

#[no_mangle]
pub extern "C" fn dummy_add(a: u64, b: u64) -> u64 {
    a.wrapping_add(b)
}
