/// This function is so RustFire can print out errors.
#[allow(dead_code)]
pub fn df_err(out: &str) {
    let red = "\x1b[31;1m";
    let reset = "\x1b[0m";
    println!("  {red}Error{reset} {out}");
}

/// This function is so RustFire can print out messages.
#[allow(dead_code)]
pub fn df_print(out: &str) {
    let blue = "\x1b[34;1m";
    let reset = "\x1b[0m";
    println!("  {blue}DF{reset} {out}");
}

/// This function is so RustFire can print out successes.
#[allow(dead_code)]
pub fn df_success(out: &str) {
    let green = "\x1b[32;1m";
    let reset = "\x1b[0m";
    println!("  {green}Success{reset} {out}");
}

#[allow(dead_code)]
#[macro_export]
macro_rules! build_set {
    ($m:expr) => {
        let blue = "\x1b[34;1m";
        let reset = "\x1b[0m";
        println!("\n{blue}{}\n{reset}", $m);
        unsafe { $crate::rustfire_current_name = format!("RustFire >> {}", $m); }
    }
}