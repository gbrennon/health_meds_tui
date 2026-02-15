use crate::core::medications_stock::{MedicationsStock};
use crate::core::medications_stock_repository::MedicationsStockRepository;
use std::fs;
use std::path::Path;

/// JSON implementation of MedicationsStockRepository
pub struct JsonMedicationsStockRepository {
    pub path: String,
}

impl JsonMedicationsStockRepository {
    pub fn new(path: &str) -> Self {
        Self { path: path.to_string() }
    }
}

impl MedicationsStockRepository for JsonMedicationsStockRepository {
    fn save(&mut self, stock: MedicationsStock) {
        let json = serde_json::to_string_pretty(&stock).unwrap();
        fs::write(&self.path, json).unwrap();
    }

    fn get(&self) -> Option<MedicationsStock> {
        if !Path::new(&self.path).exists() {
            return None;
        }
        let data = fs::read_to_string(&self.path).unwrap();
        serde_json::from_str(&data).ok()
    }
}
