use crate::types::item::Item;

pub fn internal_make_game_action(action_name: &str, items: Vec<Item>) {
    let bl = crate::types::block::Block { 
        id: "block",
        direct: "not-applicable",
        block: String::from("game_action"), 
        action: String::from(action_name), 
        items: items, 
        data: String::new() 
    }; 
    unsafe { 
        crate::code_blocks.push(bl); 
    };
    crate::build::build::check_for_lim();
}

#[macro_export]
macro_rules! game_action {
    (SetBlock $($item:expr),*) => {
        internal_make_game_action("SetBlock", vec![$($item),*]);
    };
    (SetRegion $($item:expr),*) => {
        internal_make_game_action("SetBlock", vec![$($item),*]);
    };
    (SpawnMob $($item:expr),*) => {
        internal_make_game_action("SetBlock", vec![$($item),*]);
    };
}