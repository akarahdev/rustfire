pub mod types;
pub mod code;
pub mod build;

use crate::types::block::Block;
// use crate::code::*;

pub extern crate base64;
pub extern crate clipboard;
pub extern crate libflate;
pub extern crate minifier;

#[allow(non_upper_case_globals, dead_code)]
pub static mut code_blocks: Vec<Block> = vec![];
#[allow(non_upper_case_globals, dead_code)]
pub static mut code_block_string: String = String::new();

