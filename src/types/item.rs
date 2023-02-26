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
    VanillaItem(&'static str),
    /// Vanilla MC item. String is the id. I32 is amount.
    VanillaItemStack(&'static str, i32),
    /// Custom item from the Rustfire Database.
    CustomItem(&'static str),
    /// Text Item
    Text(&'static str),
    /// Number Item
    Number(i32),
    /// Variable Item
    GlobalVariable(&'static str),
    /// Variable Item
    LocalVariable(&'static str),
    /// Variable Item
    SavedVariable(&'static str),
    /// Location Item
    Location(i32, i32, i32),
    /// Location Item with more fields
    ExtendedLocation(i32, i32, i32, i32, i32),
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
                          \"item\": \"{{Count:1b,DF_NBT:3120,id:\\\"{id}\\\",tag:{{ Unbreakable:1b }}}}\"
                        }}
                      }},
                      \"slot\": {slot}
                    }}"
                )
            }
            Item::VanillaItemStack(id, count) => {
                format!(
                "{{
                    \"item\": {{
                      \"id\": \"item\",
                      \"data\": {{
                        \"item\": \"{{Count:{count}b,DF_NBT:3120,id:\\\"{id}\\\",tag:{{ Unbreakable:1b }}}}\"
                      }}
                    }},
                    \"slot\": {slot}
                  }}"
              )
            }
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
            Item::Number(num) => {
                format!(
                    "{{
                  \"item\": {{
                    \"id\": \"num\",
                    \"data\": {{
                      \"name\": \"{num}\"
                    }}
                  }},
                  \"slot\": {slot}
                }}
                "
                )
            }
            Item::GlobalVariable(var) => {
                format!(
                    "{{
                  \"item\": {{
                    \"id\": \"var\",
                    \"data\": {{
                      \"name\": \"{var}\",
                      \"scope\": \"unsaved\"
                    }}
                  }},
                  \"slot\": {slot}
                }}"
                )
            }
            Item::LocalVariable(var) => {
                format!(
                    "{{
                \"item\": {{
                  \"id\": \"var\",
                  \"data\": {{
                    \"name\": \"{var}\",
                    \"scope\": \"local\"
                  }}
                }},
                \"slot\": {slot}
              }}"
                )
            }
            Item::SavedVariable(var) => {
                format!(
                    "{{
              \"item\": {{
                \"id\": \"var\",
                \"data\": {{
                  \"name\": \"{var}\",
                  \"scope\": \"saved\"
                }}
              }},
              \"slot\": {slot}
            }}"
                )
            }
            Item::Location(x, y, z) => {
                format!(
                    "
                {{
                  \"item\": {{
                    \"id\": \"loc\",
                    \"data\": {{
                      \"isBlock\": false,
                      \"loc\": {{
                        \"x\": {x},
                        \"y\": {y},
                        \"z\": {z},
                        \"pitch\": 0,
                        \"yaw\": 0
                      }}
                    }}
                  }}
                "
                )
            }
            Item::ExtendedLocation(x, y, z, pitch, yaw) => {
                format!(
                    "
              {{
                \"item\": {{
                  \"id\": \"loc\",
                  \"data\": {{
                    \"isBlock\": false,
                    \"loc\": {{
                      \"x\": {x},
                      \"y\": {y},
                      \"z\": {z},
                      \"pitch\": {pitch},
                      \"yaw\": {yaw}
                    }}
                  }}
                }}
              "
                )
            }
            _ => {
                todo!("unfinished item");
            }
        }
    }
}
