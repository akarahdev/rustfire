#[macro_export]
macro_rules! select {
    (default) => {
        let bl = $crate::types::block::Block {
            block: String::from("select_obj"),
            action: String::from("Default"),
            items: vec![],
            data: String::new()
        };
        unsafe {
            $crate::code_blocks.push(bl);
        };
    };
    (all_players) => {
        let bl = $crate::types::block::Block {
            block: String::from("select_obj"),
            action: String::from("AllPlayers"),
            items: vec![],
            data: String::new()
        };
        unsafe {
            $crate::code_blocks.push(bl);
        };
    };
    (player $player:expr) => {
        let bl = $crate::types::block::Block {
            block: String::from("select_obj"),
            action: String::from("PlayerName"),
            items: vec![
                $crate::types::item::Item::Text(String::from($player))
            ],
            data: String::new()
        };
        unsafe {
            $crate::code_blocks.push(bl);
        };
    };
    (entity $entity:expr) => {
        let bl = $crate::types::block::Block {
            block: String::from("select_obj"),
            action: String::from("EntityName"),
            items: vec![
                $crate::types::item::Item::Text(String::from($entity))
            ],
            data: String::new()
        };
        unsafe {
            $crate::code_blocks.push(bl);
        };
    };
}