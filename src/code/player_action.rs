/// Adds a player action to the code.
#[macro_export]
macro_rules! player_action {
    (send_message $message:expr, $target:expr) => {
        let bl = $crate::types::block::Block {
            block: String::from("player_action"),
            action: String::from("SendMessage"),
            items: vec![]
        };
        unsafe { $crate::code_blocks.push(bl); };
        println!("Send Message {} to {}", $message, $target);
    };
}