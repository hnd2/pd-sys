use pd_sys::libpd_init;

fn main() {
    unsafe {
        libpd_init();
    }
    println!("Hello, world!");
}
