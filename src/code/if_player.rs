#[allow(dead_code, unused_macros)]
#[macro_export]
macro_rules! internal_if_player {
    ($action:expr, $($item:expr),* => $blocks:block) => {
        let bl1 = crate::types::block::Block {  id: "block", direct: "not-applicable", block: String::from("if_player"),  action: String::from($action), items: $items, data: String::new() };
        let bl2 = crate::types::block::Block { id: "bracket", direct: "open", block: String::from(""), action: String::from(""), items: vec![], data: String::new() }
        let bl3 = crate::types::block::Block { id: "bracket", direct: "close", block: String::from(""), action: String::from(""), items: vec![], data: String::new() }
        unsafe { $crate::code_blocks.push(bl1); $crate::code_blocks.push(bl2); }
        $blocks
        unsafe { $crate::code_blocks.push(bl3); }
    }
}

#[macro_export]
macro_rules! if_player {
    (IsSneaking $($item:expr),* => $blocks:block) => {
        let action = "IsSneaking";
        let bl1 = crate::types::block::Block {  id: "block", direct: "not-applicable", block: String::from("if_player"),  action: String::from(action), items: vec![$( $item ),*], data: String::new() };
        let bl2 = crate::types::block::Block { id: "bracket", direct: "open", block: String::from(""), action: String::from(""), items: vec![], data: String::new() };
        let bl3 = crate::types::block::Block { id: "bracket", direct: "close", block: String::from(""), action: String::from(""), items: vec![], data: String::new() };
        unsafe { $crate::code_blocks.push(bl1); $crate::code_blocks.push(bl2); }
        $blocks
        unsafe { $crate::code_blocks.push(bl3); }
    }
}