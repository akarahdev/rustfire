/// Adds a player event to the code.
/// Valid events include:
/// ```
/// - join
/// - leave
/// - right_click
/// - left_click
/// ```
pub fn internal_player_event(action_name: &'static str) {
    let bl = crate::types::block::Block {
        id: "block",
        direct: "not-applicable",
        block: String::from("event"),
        action: String::from(action_name),
        items: vec![],
        data: String::new(),
    };
    unsafe {
        crate::code_blocks.push(bl);
    };
}

#[macro_export]
macro_rules! player_event {
    (Join) => {
        internal_player_event("Join");
    };
    (Leave) => {
        internal_player_event("Leave");
    };
    (RightClick) => {
        internal_player_event("RightClick");
    };
    (LeftClick) => {
        internal_player_event("LeftClick");
    };
    (TakeDamage) => {
        internal_player_event("PlayerTakeDmg");
    };
    (DamagePlayer) => {
        internal_player_event("PlayerDmgPlayer");
    };
    (KillPlayer) => {
        internal_player_event("KillPlayer");
    };
}
