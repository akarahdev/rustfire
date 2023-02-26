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
    (Define $name:expr) => {
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
    (Call $name:expr) => {
        let bl = $crate::types::block::Block {
            block: String::from("call_func"),
            action: String::from(""),
            items: vec![],
            data: String::from($name),
        };
        unsafe {
            $crate::code_blocks.push(bl);
        };
    };
    //  $($item:expr),*
    // $($item),*
    (CallParams $func_name:expr, $($item:expr),*) => {
        let bl1 = $crate::types::block::Block {
            block: String::from("set_var"),
            action: String::from("CreateList"),
            items: vec![
                $crate::types::item::Item::LocalVariable("functionInputParams"),
                $($item),*
            ],
            data: String::from($func_name),
        };
        let bl2 = $crate::types::block::Block {
            block: String::from("call_func"),
            action: String::from(""),
            items: vec![],
            data: String::from($func_name),
        };
        unsafe {
            $crate::code_blocks.push(bl1);
            $crate::code_blocks.push(bl2);
        };
    };
}
