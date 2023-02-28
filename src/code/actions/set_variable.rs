use crate::types::item::Item;

pub fn internal_make_set_variable(action_name: &str, items: Vec<Item>) {
    let bl = crate::types::block::Block { 
        id: "block",
        direct: "not-applicable",
        block: String::from("set_var"), 
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
macro_rules! set_variable {
    (SetTo $($item:expr),*) => {
        internal_make_set_variable("=", vec![$($item),*])
    }
}