/*

(send_message $($message:expr),*) => {
        let bl = $crate::types::block::Block {
            block: String::from("player_action"),
            action: String::from("SendMessage"),
            items: vec![
                $($crate::types::item::Item::Text(String::from($message)),)*
            ]
        };
        unsafe {
            $crate::code_blocks.push(bl);
        };
    };
*/

#[macro_export]
macro_rules! function {
    (define $name:expr) => {
        let bl = $crate::types::block::Block {
            block: String::from("func"),
            action: String::from(""),
            items: vec![],
            data: String::from($name),
        };
        unsafe {
            $crate::code_blocks.push(bl);
        };
    };
    (call $name:expr) => {
        let bl = $crate::types::block::Block {
            block: String::from("call_func"),
            action: String::from(""),
            items: vec![],
            data: String::from($name),
        };
        unsafe {
            $crate::code_blocks.push(bl);
        };
    }
}