use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum ItemType {
    Paper,
    Printer,
    Ink,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InventoryItem {
    pub item_type: ItemType,
    pub quantity: u32,
}
