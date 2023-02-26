/// Intellisense enum.
pub enum PlayerAction {
    SendMessage,
    SendMessageSequence,
    SendTitle,
    SendActionBar,

    GiveItems,
    SetHotbar,

    Damage,
    Heal,
    SetFoodLevel,
}

/// Adds a player action to the code.
#[macro_export]
macro_rules! player_action {
    (SendMessage $($item:expr),*) => {
        let action_name = "SendMessage";
        let bl = crate::types::block::Block { block: String::from("player_action"), action: String::from(action_name), items: vec![$($item),*], data: String::new() }; unsafe { crate::code_blocks.push(bl); };
    };
    (SendMessageSequence $($item:expr),*) => {
        let action_name = "SendMessageSeq";
        let bl = crate::types::block::Block { block: String::from("player_action"), action: String::from(action_name), items: vec![$($item),* ], data: String::new() }; unsafe { crate::code_blocks.push(bl); };
    };
    (SendTitle $($item:expr),*) => {
        let action_name = "SendTitle";
        let bl = crate::types::block::Block { block: String::from("player_action"), action: String::from(action_name), items: vec![$($item),* ], data: String::new() }; unsafe { crate::code_blocks.push(bl); };
    };
    (SendActionBar $($item:expr),*) => {
        let action_name = "ActionBar";
        let bl = crate::types::block::Block { block: String::from("player_action"), action: String::from(action_name), items: vec![$($item),* ], data: String::new() }; unsafe { crate::code_blocks.push(bl); };
    };
    (GiveItems $($item:expr),*) => {
        let action_name = "GiveItems";
        let bl = crate::types::block::Block { block: String::from("player_action"), action: String::from(action_name), items: vec![$($item),* ], data: String::new() }; unsafe { crate::code_blocks.push(bl); };
    };
    (SetHotbar $($item:expr),*) => {
        let action_name = "SetHotbar";
        let bl = crate::types::block::Block { block: String::from("player_action"), action: String::from(action_name), items: vec![$($item),* ], data: String::new() }; unsafe { crate::code_blocks.push(bl); };
    };
    (SetArmor $($item:expr),*) => {
        let action_name = "SetArmor";
        let bl = crate::types::block::Block { block: String::from("player_action"), action: String::from(action_name), items: vec![$($item),* ], data: String::new() }; unsafe { crate::code_blocks.push(bl); };
    };
    (Damage $($item:expr),*) => {
        let action_name = "Damage";
        let bl = crate::types::block::Block { block: String::from("player_action"), action: String::from(action_name), items: vec![$($item),* ], data: String::new() }; unsafe { crate::code_blocks.push(bl); };
    };
    (Heal $($item:expr),*) => {
        let action_name = "Heal";
        let bl = crate::types::block::Block { block: String::from("player_action"), action: String::from(action_name), items: vec![$($item),*], data: String::new() }; unsafe { crate::code_blocks.push(bl); };
    };
    (SetFoodLevel $($item:expr),*) => {
        let action_name = "SetFoodLevel";
        let bl = crate::types::block::Block { block: String::from("player_action"), action: String::from(action_name), items: vec![$($item),*], data: String::new() }; unsafe { crate::code_blocks.push(bl); };
    };
}
