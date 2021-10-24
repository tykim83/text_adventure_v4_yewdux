#[derive(Debug, Clone)]
pub struct Item {
    pub id: ItemType,
    pub name: String,
    pub description: String,
    pub can_pick_up: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum ItemType {
    Key,
    Table,
    Door,
}