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
    (join) => {
        let bl = $crate::types::block::Block {
            block: String::from("event"),
            action: String::from("Join"),
            items: vec![]
        };
        unsafe { $crate::code_blocks.push(bl); };
    };
    (leave) => {
        let bl = $crate::types::block::Block {
            block: String::from("event"),
            action: String::from("Leave"),
            items: vec![]
        };
        unsafe { $crate::code_blocks.push(bl); };
    };
    (right_click) => {
        let bl = $crate::types::block::Block {
            block: String::from("event"),
            action: String::from("RightClick"),
            items: vec![]
        };
        unsafe { $crate::code_blocks.push(bl); };
    };
    (left_click) => {
        let bl = $crate::types::block::Block {
            block: String::from("event"),
            action: String::from("LeftClick"),
            items: vec![]
        };
        unsafe { $crate::code_blocks.push(bl); };
    };
}