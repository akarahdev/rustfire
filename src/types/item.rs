/// This struct is for adding vanilla items to the chests.
/// In DF's JSON, an item looks something like this.
/// ```
/// {
/// "item": {
///     "id": "item",
///     "data": {
///       "item": "{Count:1b,DF_NBT:3120,id:\"minecraft:diamond_sword\",tag:{Damage:0}}"
///     }
///   },
///   "slot": 0
/// }
/// ```
#[derive(Debug)]
pub enum Item {
    /// Vanilla MC item. String is the id.
    VanillaItem(String),
    /// Custom item from the Rustfire Database.
    CustomItem(String),
    /// Text Item
    Text(String),
    /// Number Item
    Number(i32),
    /// Variable Item
    Variable(String)

}

impl Item {
    pub fn as_str(&self, slot: i32) -> String {
        /*  */
        match &self {
            Item::VanillaItem(id) => {
                format!(
                  "{{
                      \"item\": {{
                        \"id\": \"item\",
                        \"data\": {{
                          \"item\": \"{{Count:1b,DF_NBT:3120,id:\\\"{id}\\\",tag:{{ Damage:1b }}}}\"
                        }}
                      }},
                      \"slot\": {slot}
                    }}"
                )
            },
            Item::Text(text) => {
                let value2 = text.replace("&", "§");
                format!(
                    "{{
                      \"item\": {{
                        \"id\": \"txt\",
                        \"data\": {{
                          \"name\": \"{value2}\"
                        }}
                      }},
                      \"slot\": {slot}
                    }}"
                )
            }
            _ => { todo!("unfinished item"); }
        }
    }
}

pub fn item_from_string(input: std::string::String) -> Item {
    if input.starts_with("vanilla::") {
        let input2 = input.replace("vanilla::", "");
        return Item::VanillaItem(format!("minecraft:{input2}"));
    }
    if input.starts_with("custom::") {
      let input2 = input.replace("custom::", "");
      return Item::VanillaItem(format!("{input2}"));
    }
    if input.starts_with("text::") {
      let input2 = input.replace("text::", "");
      return Item::Text(format!("{input2}"));
    }
    Item::VanillaItem(String::from("air"))
}


