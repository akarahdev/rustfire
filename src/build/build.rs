use base64::Engine;

use crate::print::df_warn;
use crate::function;

/// Struct for managing build flags
pub struct BuildFlags {
    pub template_name: &'static str,
    pub build: bool,
    pub length: i32
}

/// Holds flags for build
#[allow(dead_code)]
pub fn rf_build_flags(bf: BuildFlags) {
    unsafe {
        crate::rustfire_current_name = bf.template_name.to_string();
        crate::rustfire_is_building = bf.build;
        crate::rustfire_max_length = bf.length;
        crate::rustfire_build_flags = bf;
    }
}

#[allow(dead_code)]
pub fn check_for_lim() {
    #[allow(unused_unsafe)]
    unsafe {    
        println!("Code Blocks: {} // Length: {}", crate::rustfire_code_blocks, crate::rustfire_max_length);
        crate::rustfire_code_blocks += 1;
        if crate::rustfire_code_blocks >= crate::rustfire_max_length  {
            df_warn("Max code line length hit. Creating new line.");
            crate::rustfire_code_blocks = 0;
            crate::rustfire_code_warps += 1;
            function!(Call format!("warp::{}", crate::rustfire_code_warps));
            build();
            crate::rustfire_current_name = crate::rustfire_current_name.replace(&format!("[w{}] ", crate::rustfire_code_warps-1), "");
            crate::rustfire_current_name = format!("[w{}] {}", crate::rustfire_code_warps, crate::rustfire_current_name);
            function!(Define format!("warp::{}", crate::rustfire_code_warps));
        }
    }
}

/// Macro for creating a code line.
/// The first argument should be a `BuildFlags` struct.
/// The second should be a callback of the code you wish to run.
/// ```
/// df!(
///    BuildFlags {
///        template_name: "Join with Parameters",
///        build: true
///    },
///    {
///        player_event!(Join);
///        player_action!(SendMessage Item::Text("Hello %default!"));
///    }
/// );
/// ```
#[macro_export]
macro_rules! df {
    ($build_flags:expr, $body:block) => {
        rf_build_flags($build_flags);
        if($build_flags.build == false) {
            return;
        }
        $body
        build();
    };
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
        let send_over = format!("{{\"type\":\"template\",\"source\":\"RustFire\",\"data\": \"{{\\\"name\\\":\\\"[RF] {name}\\\",\\\"data\\\":\\\"{b64}\\\"}}\"}}");
        println!("\nFinished!");
        crate::code_blocks = vec![];
        crate::code_block_string = String::new();
        std::process::Command::new("python")
            .arg("send.py")
            .arg(send_over)
            .output()
            .expect("failed to execute process");
    }
}
