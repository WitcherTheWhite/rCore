use core::panic::PanicInfo;

use crate::println;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "Paniked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().as_str().unwrap()
        );
    } else {
        println!("Panicked: {}", info.message().as_str().unwrap());
    }
    loop {}
}
