#[unsafe(no_mangle)]
pub extern "C" fn run_the_project() -> i32 {
    println!("Project finished successfully!");
    0 // Return 0 for success, just like in C's main()
}

