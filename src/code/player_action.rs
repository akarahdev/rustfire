/// Adds a player action to the code.
/// Valid actions include:
/// ```
/// - send_message [String] => SendMessage
/// - give_item [Item] => GiveItems
/// ```
#[macro_export]
macro_rules! player_action {
    (send_message $($message:expr),*) => {
        let bl = $crate::types::block::Block {
            block: String::from("player_action"),
            action: String::from("SendMessage"),
            items: vec![
                $($crate::types::item::item_from_string(String::from($message)),)*
            ],
            data: String::new()
        };
        unsafe {
            $crate::code_blocks.push(bl);
        };
    };
    (give_item $($item:expr),*) => {
        let bl = $crate::types::block::Block {
            block: String::from("player_action"),
            action: String::from("GiveItems"),
            items: vec![
                $(
                    $crate::types::item::item_from_string(String::from(format!("{}", $item.trim()))),
                )*
            ],
            data: String::new()
        };
        unsafe {
            $crate::code_blocks.push(bl);
        };
    };
}
