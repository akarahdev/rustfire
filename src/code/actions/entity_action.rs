use crate::types::item::Item;

pub fn internal_make_entity_action(action_name: &str, items: Vec<Item>) {
    let bl = crate::types::block::Block { 
        id: "block",
        direct: "not-applicable",
        block: String::from("entity_action"), 
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
macro_rules! entity_action {
    (Heal $($item:expr),*) => {
        internal_make_entity_action("Heal", vec![$($item),*]);
    };
    (Damage $($item:expr),*) => {
        internal_make_entity_action("Damage", vec![$($item),*]);
    };
}