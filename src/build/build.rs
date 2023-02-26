/// Builds the code you have created.
#[macro_export]
macro_rules! legacy_build {
    () => {
        unsafe {
            let mut index = 0;
            for block in &$crate::code_blocks {
                index += 1;
                $crate::code_block_string = format!("{}{}", $crate::code_block_string, block.as_str());
                if index != $crate::code_blocks.len() {
                    $crate::code_block_string = format!("{},", $crate::code_block_string);
                }
            }
            $crate::code_block_string = format!("{{\"blocks\":[{}]}}", code_block_string);

            let mut data = &$crate::code_block_string;
            let mut data_1 = minifier::json::minify(data.as_str()).to_string();
            let mut data_as_bytes = data_1.as_bytes();
            let mut encoder = $crate::libflate::gzip::Encoder::new(Vec::new()).unwrap();
            std::io::copy(&mut data_as_bytes, &mut encoder).unwrap();
            let compressed = encoder.finish().into_result().unwrap();

            use rustfire::base64::Engine;
            let b64 = $crate::base64::engine::general_purpose::STANDARD.encode(&compressed);

            // println!("{:?}", compressed);
            let br = format!("@rf -a {{\"author\":\"RustFire\",\"name\":\"RustFire Code Template\",\"version\":1,\"code\":\"");

            let mut join = format!("{}{}", br, b64);
            join.push_str("\"}");

            for r in 1..60 {
                if join.len() > r * 240 {
                    join.insert_str(r * 240, "\n\n@rf -a ");
                }
            }

            println!("{}", join.clone().replace("\n\n\n/txt ", ""));
            println!("\nFinished!");

            $crate::code_blocks = vec![];
            $crate::code_block_string = String::new();
        }
    };
}

#[macro_export]
macro_rules! build {
    () => {
        unsafe {
            let mut index = 0;
            for block in &$crate::code_blocks {
                index += 1;
                $crate::code_block_string = format!("{}{}", $crate::code_block_string, block.as_str());
                if index != $crate::code_blocks.len() {
                    $crate::code_block_string = format!("{},", $crate::code_block_string);
                }
            }
            $crate::code_block_string = format!("{{\"blocks\":[{}]}}", code_block_string);

            let mut data = &$crate::code_block_string;
            let mut data_1 = minifier::json::minify(data.as_str()).to_string();
            let mut data_as_bytes = data_1.as_bytes();
            let mut encoder = $crate::libflate::gzip::Encoder::new(Vec::new()).unwrap();
            std::io::copy(&mut data_as_bytes, &mut encoder).unwrap();
            let compressed = encoder.finish().into_result().unwrap();

            use rustfire::base64::Engine;
            let b64 = $crate::base64::engine::general_purpose::STANDARD.encode(&compressed);
            // replace data with b64
            let name = &$crate::rustfire_current_name;
            let send_over = format!("{{\"type\":\"template\",\"source\":\"RustFire\",\"data\": \"{{\\\"name\\\":\\\"{name}\\\",\\\"data\\\":\\\"{b64}\\\"}}\"}}");
            println!("\nFinished!");

            $crate::code_blocks = vec![];
            $crate::code_block_string = String::new();

            std::process::Command::new("python")
                .arg("send.py")
                .arg(send_over)
                .output()
                .expect("failed to execute process")
        }
    };
}
