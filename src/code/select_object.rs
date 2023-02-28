pub fn make_select_object(action_name: &'static str) {
    let bl = crate::types::block::Block { id: "block", direct: "not-applicable", block: String::from("select_obj"), action: String::from(action_name), items: vec![], data: String::new() };
    unsafe { crate::code_blocks.push(bl);};
}

#[macro_export]
macro_rules! select {
    (Default) => {
        make_select_object("Default");
    };
    (AllPlayers) => {
        make_select_object("AllPlayers");
    };
    (Player $player:expr) => {
        let bl = $crate::types::block::Block {
            id: "block", 
            direct: "not-applicable",
            block: String::from("select_obj"),
            action: String::from("PlayerName"),
            items: vec![
                $player
            ],
            data: String::new()
        };
        unsafe {
            $crate::code_blocks.push(bl);
        };
    };
    (Entity $entity:expr) => {
        let bl = $crate::types::block::Block {
            id: "block", 
            direct: "not-applicable",
            block: String::from("select_obj"),
            action: String::from("EntityName"),
            items: vec![
                $entity
            ],
            data: String::new()
        };
        unsafe {
            $crate::code_blocks.push(bl);
        };
    };
}

#[macro_export]
macro_rules! target {
    (AllPlayers) => {
        let action = "AllPlayers";
        let bl = $crate::types::block::Block {
            id: "block", 
            direct: "not-applicable",
            block: String::from("RF_RESERVED::set_target"),
            action: String::from(action),
            items: vec![],
            data: String::new(),
        };
        unsafe {
            $crate::code_blocks.push(bl);
        };
    };
    (Default) => {
        let action = "";
        let bl = $crate::types::block::Block {
            id: "block", 
            direct: "not-applicable",
            block: String::from("RF_RESERVED::set_target"),
            action: String::from(action),
            items: vec![],
            data: String::new(),
        };
        unsafe {
            $crate::code_blocks.push(bl);
        };
    };
    (Killer) => {
        let action = "Killer";
        let bl = $crate::types::block::Block {
            id: "block", 
            direct: "not-applicable",
            block: String::from("RF_RESERVED::set_target"),
            action: String::from(action),
            items: vec![],
            data: String::new(),
        };
        unsafe {
            $crate::code_blocks.push(bl);
        };
    };
    (Victim) => {
        let action = "Victim";
        let bl = $crate::types::block::Block {
            id: "block", 
            direct: "not-applicable",
            block: String::from("RF_RESERVED::set_target"),
            action: String::from(action),
            items: vec![],
            data: String::new(),
        };
        unsafe {
            $crate::code_blocks.push(bl);
        };
    };
    (Damager) => {
        let action = "Damager";
        let bl = $crate::types::block::Block {
            id: "block", 
            direct: "not-applicable",
            block: String::from("RF_RESERVED::set_target"),
            action: String::from(action),
            items: vec![],
            data: String::new(),
        };
        unsafe {
            $crate::code_blocks.push(bl);
        };
    };
}
