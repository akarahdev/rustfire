use crate::types::item::Item;

/// In DF's JSON, a block looks something like this.
/// ```
/// {
///     "id": "block",
///     "block": "player_action",
///     "args": {
///       "items": [
///         ...
///       ]
///     },
///     "action": "GiveItems"
///  }
/// ```
#[allow(dead_code)]
#[derive(Debug)]
pub struct Block {
    pub block: String,
    pub action: String,
    pub items: Vec<Item>
}

#[allow(dead_code)]
impl Block {
    pub fn as_str(&self) -> String {
        String::from(format!(
            "
            {{
             \"id\": \"block\",
             \"block\": \"{}\",
             \"args\": {{
               \"items\": []
             }},
             \"action\": \"{}\"
          }}
        ",
            self.block, self.action
        ))
    }
}