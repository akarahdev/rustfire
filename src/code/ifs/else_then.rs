#[macro_export]
macro_rules! else_then {
    ($blocks:block) => {
        let action = "";
        let bl1 = crate::types::block::Block {  id: "block", direct: "not-applicable", block: String::from("else"),  action: String::from(action), items: vec![], data: String::new() }; let bl2 = crate::types::block::Block { id: "bracket", direct: "open", block: String::from(""), action: String::from(""), items: vec![], data: String::new() }; let bl3 = crate::types::block::Block { id: "bracket", direct: "close", block: String::from(""), action: String::from(""), items: vec![], data: String::new() };
        unsafe { $crate::code_blocks.push(bl1); $crate::code_blocks.push(bl2); } $blocks unsafe { $crate::code_blocks.push(bl3); }
    };
}