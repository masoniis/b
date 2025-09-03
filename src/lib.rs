#[unsafe(no_mangle)]
pub extern "C" fn run_the_project() -> i32 {
    println!("Hello from Rust!");
    0 // Return 0 for success, just like in C's main()
}
