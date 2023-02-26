/// Adds a player event to the code.
/// Valid events include:
/// ```
/// - join
/// - leave
/// - right_click
/// - left_click
/// ```
#[macro_export]
macro_rules! player_event {
    (Join) => {
        let action_name = "Join";
        let bl = $crate::types::block::Block {
            block: String::from("event"),
            action: String::from(action_name),
            items: vec![],
            data: String::new(),
        };
        unsafe {
            $crate::code_blocks.push(bl);
        };
    };
    (Leave) => {
        let action_name = "Leave";
        let bl = $crate::types::block::Block {
            block: String::from("event"),
            action: String::from(action_name),
            items: vec![],
            data: String::new(),
        };
        unsafe {
            $crate::code_blocks.push(bl);
        };
    };
    (RightClick) => {
        let action_name = "RightClick";
        let bl = $crate::types::block::Block {
            block: String::from("event"),
            action: String::from(action_name),
            items: vec![],
            data: String::new(),
        };
        unsafe {
            $crate::code_blocks.push(bl);
        };
    };
    (LeftClick) => {
        let action_name = "LeftClick";
        let bl = $crate::types::block::Block {
            block: String::from("event"),
            action: String::from(action_name),
            items: vec![],
            data: String::new(),
        };
        unsafe {
            $crate::code_blocks.push(bl);
        };
    };
    (TakeDamage) => {
        let action_name = "PlayerTakeDmg";
        let bl = $crate::types::block::Block {
            block: String::from("event"),
            action: String::from(action_name),
            items: vec![],
            data: String::new(),
        };
        unsafe {
            $crate::code_blocks.push(bl);
        };
    };
    (DamagePlayer) => {
        let action_name = "PlayerDmgPlayer";
        let bl = $crate::types::block::Block {
            block: String::from("event"),
            action: String::from(action_name),
            items: vec![],
            data: String::new(),
        };
        unsafe {
            $crate::code_blocks.push(bl);
        };
    };
    (KillPlayer) => {
        let action_name = "KillPlayer";
        let bl = $crate::types::block::Block {
            block: String::from("event"),
            action: String::from(action_name),
            items: vec![],
            data: String::new(),
        };
        unsafe {
            $crate::code_blocks.push(bl);
        };
    };
}
