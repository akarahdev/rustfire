use crate::{rustfire_current_target, types::item::Item};

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
    pub id: &'static str,
    pub direct: &'static str,
    pub block: String,
    pub action: String,
    pub data: String,
    pub items: Vec<Item>,
}

#[allow(dead_code)]
impl Block {
    pub fn as_str(&self) -> String {
        let mut item_string = String::new();
        let mut item_index = 0;
        for item in &self.items {
            item_index += 1;
            item_string.push_str(&item.as_str(item_index - 1).to_owned());
            if &(item_index as usize) != &self.items.len() {
                item_string.push(',');
            }
        }

        let minified = minifier::json::minify(item_string.as_str()).to_string();
        // $crate::rustfire_current_target
        unsafe {
            String::from(format!(
                "
            {{
             \"id\": \"{}\",
             \"direct\": \"{}\",
             \"type\": \"norm\",
             \"block\": \"{}\",
             \"args\": {{
               \"items\": [
                    {}
               ]
             }},
             \"action\": \"{}\",
             \"data\": \"{}\",
             \"target\": \"{}\"
          }}
        ",
                self.id, self.direct, self.block, minified, self.action, self.data, rustfire_current_target
            ))
        }
    }
}
