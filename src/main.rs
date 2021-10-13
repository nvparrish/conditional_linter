/// This is an unused enumeration with dead code
// #[allow(dead_code)] // Just allow dead code
#[cfg_attr(feature = "quiet_warnings", allow(dead_code))] // Only allow dead_code if quiet_warnings feature is enabled
enum DeadEnum {
    FOO,
    BAR,
    BAZ,
}

fn main() {
    println!("Hello, world!");
}
