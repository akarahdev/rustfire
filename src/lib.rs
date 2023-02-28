pub mod build;
pub mod code;
pub mod print;
pub mod types;

use build::build::BuildFlags;

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

#[allow(non_upper_case_globals, dead_code)]
pub static mut rustfire_functions: Vec<String> = Vec::new();

#[allow(non_upper_case_globals, dead_code)]
pub static mut rustfire_current_name: String = String::new();

#[allow(non_upper_case_globals, dead_code)]
pub static mut rustfire_current_target: String = String::new();

#[allow(non_upper_case_globals, dead_code)]
pub static mut rustfire_is_building: bool = false;

#[allow(non_upper_case_globals, dead_code)]
pub static mut rustfire_code_warps: i32 = 0;

#[allow(non_upper_case_globals, dead_code)]
pub static mut rustfire_code_blocks: i32 = 0;

#[allow(non_upper_case_globals, dead_code)]
pub static mut rustfire_max_length: i32 = 0;

#[allow(non_upper_case_globals, dead_code)]
pub static mut rustfire_build_flags: BuildFlags = BuildFlags {
    template_name: "unnamed",
    build: true,
    length: 50,
};

#[macro_export]
macro_rules! rf_imports {
    () => {
        use rustfire::*;
        use rustfire::build::build::{BuildFlags, rf_build_flags};
        use rustfire::types::item::Item::*;
        use rustfire::build::build::build;
        use rustfire::code::actions::player_action::internal_make_player_action;
        use rustfire::code::actions::entity_action::internal_make_entity_action;
        use rustfire::code::actions::set_variable::internal_make_set_variable;
        use rustfire::code::actions::game_action::internal_make_game_action;
        use rustfire::code::events::player_event::internal_player_event;
    };
}
