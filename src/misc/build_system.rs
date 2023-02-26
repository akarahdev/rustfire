#[macro_export]
macro_rules! rf_fn {
    ($func:expr) => {
        unsafe { $crate::rustfire_functions.push(String::from(format!("{}", $func))); }
    };
}

#[macro_export]
macro_rules! init_rf_build_system {
    () => {
        unsafe { 
            let mut index = 0;
            for str in &$crate::rustfire_functions {
                index += 1;
                println!("{index}. {str}");
            }
        }

        println!("Choose a function:");
        let mut input_text = String::new();
        std::io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

        let trimmed = input_text.trim();
        match trimmed.parse::<u32>() {
            Ok(i) => unsafe {
                if i < ($crate::rustfire_functions.len() as u32) {
                    let ind = $crate::rustfire_functions.get(i as usize);
                    rf_build_call_func!(ind.unwrap());
                }
            },
            Err(..) => println!("this was not an integer: {}", trimmed),
        };
    }
}

#[macro_export]
macro_rules! rf_build_call_func {
    ($func:expr) => {
        $func();
    };
}