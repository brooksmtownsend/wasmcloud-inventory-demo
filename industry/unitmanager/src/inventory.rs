use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct InventoryItem {
    pub item_type: String,
    pub quantity: i32,
}

impl InventoryItem {
    pub fn new(item_type: String, quantity: i32) -> Self {
        Self {
            item_type,
            quantity,
        }
    }

    pub fn item_type(&self) -> String {
        self.item_type.to_lowercase().replace(" ", "_")
    }

    pub fn storage_key(&self) -> String {
        format!("inventory:{}", self.item_type())
    }
}
