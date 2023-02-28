#[macro_export]
macro_rules! if_player {
    (IsHolding $($item:expr),* => $blocks:block) => {
        let action = "IsHolding";
        let bl1 = crate::types::block::Block {  id: "block", direct: "not-applicable", block: String::from("if_player"),  action: String::from(action), items: vec![$( $item ),*], data: String::new() }; let bl2 = crate::types::block::Block { id: "bracket", direct: "open", block: String::from(""), action: String::from(""), items: vec![], data: String::new() }; let bl3 = crate::types::block::Block { id: "bracket", direct: "close", block: String::from(""), action: String::from(""), items: vec![], data: String::new() };
        unsafe { $crate::code_blocks.push(bl1); $crate::code_blocks.push(bl2); } $blocks unsafe { $crate::code_blocks.push(bl3); }
    };
    (IsSneaking => $blocks:block) => {
        let action = "IsSneaking";
        let bl1 = crate::types::block::Block {  id: "block", direct: "not-applicable", block: String::from("if_player"),  action: String::from(action), items: vec![], data: String::new() }; let bl2 = crate::types::block::Block { id: "bracket", direct: "open", block: String::from(""), action: String::from(""), items: vec![], data: String::new() }; let bl3 = crate::types::block::Block { id: "bracket", direct: "close", block: String::from(""), action: String::from(""), items: vec![], data: String::new() };
        unsafe { $crate::code_blocks.push(bl1); $crate::code_blocks.push(bl2); } $blocks unsafe { $crate::code_blocks.push(bl3); }
    };
    (IsSprinting => $blocks:block) => {
        let action = "IsSprinting";
        let bl1 = crate::types::block::Block {  id: "block", direct: "not-applicable", block: String::from("if_player"),  action: String::from(action), items: vec![], data: String::new() }; let bl2 = crate::types::block::Block { id: "bracket", direct: "open", block: String::from(""), action: String::from(""), items: vec![], data: String::new() }; let bl3 = crate::types::block::Block { id: "bracket", direct: "close", block: String::from(""), action: String::from(""), items: vec![], data: String::new() };
        unsafe { $crate::code_blocks.push(bl1); $crate::code_blocks.push(bl2); } $blocks unsafe { $crate::code_blocks.push(bl3); }
    };
    (IsGliding => $blocks:block) => {
        let action = "IsGliding";
        let bl1 = crate::types::block::Block {  id: "block", direct: "not-applicable", block: String::from("if_player"),  action: String::from(action), items: vec![], data: String::new() }; let bl2 = crate::types::block::Block { id: "bracket", direct: "open", block: String::from(""), action: String::from(""), items: vec![], data: String::new() }; let bl3 = crate::types::block::Block { id: "bracket", direct: "close", block: String::from(""), action: String::from(""), items: vec![], data: String::new() };
        unsafe { $crate::code_blocks.push(bl1); $crate::code_blocks.push(bl2); } $blocks unsafe { $crate::code_blocks.push(bl3); }
    };
    (IsFlying => $blocks:block) => {
        let action = "IsFlying";
        let bl1 = crate::types::block::Block {  id: "block", direct: "not-applicable", block: String::from("if_player"),  action: String::from(action), items: vec![], data: String::new() }; let bl2 = crate::types::block::Block { id: "bracket", direct: "open", block: String::from(""), action: String::from(""), items: vec![], data: String::new() }; let bl3 = crate::types::block::Block { id: "bracket", direct: "close", block: String::from(""), action: String::from(""), items: vec![], data: String::new() };
        unsafe { $crate::code_blocks.push(bl1); $crate::code_blocks.push(bl2); } $blocks unsafe { $crate::code_blocks.push(bl3); }
    };
    (IsGrounded => $blocks:block) => {
        let action = "IsGrounded";
        let bl1 = crate::types::block::Block {  id: "block", direct: "not-applicable", block: String::from("if_player"),  action: String::from(action), items: vec![], data: String::new() }; let bl2 = crate::types::block::Block { id: "bracket", direct: "open", block: String::from(""), action: String::from(""), items: vec![], data: String::new() }; let bl3 = crate::types::block::Block { id: "bracket", direct: "close", block: String::from(""), action: String::from(""), items: vec![], data: String::new() };
        unsafe { $crate::code_blocks.push(bl1); $crate::code_blocks.push(bl2); } $blocks unsafe { $crate::code_blocks.push(bl3); }
    };
}