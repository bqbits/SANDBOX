use std::sync::{Arc, Mutex};

#[derive(Clone, serde::Serialize)]
pub struct ShopStats {
    pub potions_sold: u64,
    pub gold_earned: u64,
    pub gold_per_potion: u64,
}

impl ShopStats {
    pub fn new() -> Self {
        ShopStats { potions_sold: 0, gold_earned: 0, gold_per_potion: 50 }
    }
}

pub type SharedStore = Arc<Mutex<ShopStats>>;

pub fn new_store() -> SharedStore {
    Arc::new(Mutex::new(ShopStats::new()))
}
