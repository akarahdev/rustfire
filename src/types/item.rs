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
#[allow(dead_code)]
#[derive(Debug)]
pub struct Item {
    id: String,
    nbt: String,
    lore: Vec<String>,
}

/*
{
            "item": {
              "id": "item",
              "data": {
                "item": "{Count:1b,DF_NBT:3120,id:\"minecraft:iron_sword\",tag:{Damage:0,display:{Lore:['{\"extra\":[{\"bold\":false,\"italic\":false,\"underlined\":false,\"strikethrough\":false,\"obfuscated\":false,\"color\":\"gray\",\"text\":\"Lore Line 1\"}],\"text\":\"\"}','{\"extra\":[{\"bold\":false,\"italic\":false,\"underlined\":false,\"strikethrough\":false,\"obfuscated\":false,\"color\":\"aqua\",\"text\":\"Lore Line 2\"}],\"text\":\"\"}'],Name:'{\"extra\":[{\"bold\":false,\"italic\":false,\"underlined\":false,\"strikethrough\":false,\"obfuscated\":false,\"color\":\"white\",\"text\":\"Item Name Here\"}],\"text\":\"\"}'}}}"
              }
            },
            "slot": 0
          }
 */

#[allow(dead_code)]
impl Item {
    pub fn as_str(&self) -> String {
        format!(
            "{{
                \"item\": {{
                  \"id\": \"item\",
                  \"data\": {{
                    \"item\": \"{{Count:1b,DF_NBT:3120,id:\"{}\",tag:{{ {} }}}}\"
                  }}
                }},
                \"slot\": 0
              }}",
            self.id, self.nbt
        )
    }
}