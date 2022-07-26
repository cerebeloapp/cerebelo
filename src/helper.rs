// TODO: Remove \x1b color codes when adding Windows Support.
pub fn terminate(error_s: &str, error_n: i32) -> ! {
    if error_n < 1 {
        println!("\x1b[34m[{}] Success\x1b[0m: {}", error_n, error_s);
    }
    else {
        println!("\x1b[31m[{}] Error\x1b[0m: {}", error_n, error_s);
    }
    std::process::exit(error_n)
}

pub fn debug(debug_s: &str) {
    println!("\x1b[33mDEBUG\x1b[0m: {}", debug_s)
}
