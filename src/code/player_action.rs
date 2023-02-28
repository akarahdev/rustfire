use crate::types::item::Item;

pub fn internal_make_player_action(action_name: &str, items: Vec<Item>) {
    let bl = crate::types::block::Block { 
        id: "block",
        direct: "not-applicable",
        block: String::from("player_action"), 
        action: String::from(action_name), 
        items: items, 
        data: String::new() 
    }; 
    unsafe { 
        crate::code_blocks.push(bl); 
    };
    crate::build::build::check_for_lim();
}

/// Adds a player action to the code.
#[macro_export]
macro_rules! player_action {
    (SendMessage $($item:expr),*) => {
        internal_make_player_action("SendMessage", vec![$($item),*]);
    };
    (SendMessageSequence $($item:expr),*) => {
        internal_make_player_action("SendMessageSeq", vec![$($item),*]);
    };
    (SendTitle $($item:expr),*) => {
        internal_make_player_action("SendTitle", vec![$($item),*]);
    };
    (SendActionBar $($item:expr),*) => {
        internal_make_player_action("ActionBar", vec![$($item),*]);
    };
    (GiveItems $($item:expr),*) => {
        internal_make_player_action("GiveItems", vec![$($item),*]);
    };
    (SetHotbar $($item:expr),*) => {
        internal_make_player_action("ActionBar", vec![$($item),*]);
    };
    (SetArmor $($item:expr),*) => {
        internal_make_player_action("SetArmor", vec![$($item),*]);
    };
    (Damage $($item:expr),*) => {
        internal_make_player_action("Damage", vec![$($item),*]);
    };
    (Heal $($item:expr),*) => {
        internal_make_player_action("Heal", vec![$($item),*]);
    };
    (SetFoodLevel $($item:expr),*) => {
        let action_name = "SetFoodLevel";
        let bl = crate::types::block::Block { block: String::from("player_action"), action: String::from(action_name), items: vec![$($item),*], data: String::new() }; unsafe { crate::code_blocks.push(bl); };
        crate::build::build::check_for_lim();
    };
    (Teleport $($item:expr),*) => {
        let action_name = "Teleport";
        let bl = crate::types::block::Block { block: String::from("player_action"), action: String::from(action_name), items: vec![$($item),*], data: String::new() }; unsafe { crate::code_blocks.push(bl); };
        crate::build::build::check_for_lim();
    };
    (LaunchUp $($item:expr),*) => {
        let action_name = "LaunchUp";
        let bl = crate::types::block::Block { block: String::from("player_action"), action: String::from(action_name), items: vec![$($item),*], data: String::new() }; unsafe { crate::code_blocks.push(bl); };
        crate::build::build::check_for_lim();
    };
}
