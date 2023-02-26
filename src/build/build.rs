use base64::Engine;

/// Struct for managing build flags
pub struct BuildFlags {
    pub template_name: &'static str
}

/// Holds flags for build
#[allow(dead_code)]
pub fn rf_build_flags(bf: BuildFlags) {
    unsafe {
        crate::rustfire_current_name = bf.template_name.to_string();
    }
}

/// Builds code you have created.
pub fn build() {
    unsafe {
        let mut index = 0;
        for block in &crate::code_blocks {
            index += 1;
            if block.block.starts_with("RF_RESERVED::") {
                let name = &block.block;
                let name_1 = name.replace("RF_RESERVED::", "");
                if name_1 == "set_target" {
                    let action = &block.action;
                    crate::rustfire_current_target = action.to_string();
                }
            } else {
                crate::code_block_string =
                    format!("{}{}", crate::code_block_string, block.as_str());
                if index != crate::code_blocks.len() {
                    crate::code_block_string = format!("{},", crate::code_block_string);
                }
            }
        }
        crate::code_block_string = format!("{{\"blocks\":[{}]}}", crate::code_block_string);
        let data = &crate::code_block_string;
        let data_1 = minifier::json::minify(data.as_str()).to_string();
        let mut data_as_bytes = data_1.as_bytes();
        let mut encoder = crate::libflate::gzip::Encoder::new(Vec::new()).unwrap();
        std::io::copy(&mut data_as_bytes, &mut encoder).unwrap();
        let compressed = encoder.finish().into_result().unwrap();

        let b64 = crate::base64::engine::general_purpose::STANDARD.encode(&compressed);
        // replace data with b64
        let name = &crate::rustfire_current_name;
        let send_over = format!("{{\"type\":\"template\",\"source\":\"RustFire\",\"data\": \"{{\\\"name\\\":\\\"{name}\\\",\\\"data\\\":\\\"{b64}\\\"}}\"}}");
        println!("\nFinished!");
        crate::code_blocks = vec![];
        crate::code_block_string = String::new();
        crate::rustfire_current_name = String::new();
        crate::rustfire_current_target = String::new();
        std::process::Command::new("python")
            .arg("send.py")
            .arg(send_over)
            .output()
            .expect("failed to execute process");
    }
}
