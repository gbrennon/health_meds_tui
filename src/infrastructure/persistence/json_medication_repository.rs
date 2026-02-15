use crate::core::medication::{Medication};
use crate::core::medication_repository::MedicationRepository;
use std::fs;
use std::path::Path;
use std::collections::HashMap;

/// JSON implementation of MedicationRepository
pub struct JsonMedicationRepository {
    pub path: String,
}

impl JsonMedicationRepository {
    pub fn new(path: &str) -> Self {
        Self { path: path.to_string() }
    }
}

impl MedicationRepository for JsonMedicationRepository {
    fn save(&mut self, medication: Medication) {
        let mut meds = self.list();
        meds.push(medication);
        let json = serde_json::to_string_pretty(&meds).unwrap();
        fs::write(&self.path, json).unwrap();
    }

    fn get(&self, name: &str) -> Option<Medication> {
        self.list().into_iter().find(|m| m.name == name)
    }

    fn list(&self) -> Vec<Medication> {
        if !Path::new(&self.path).exists() {
            return vec![];
        }
        let data = fs::read_to_string(&self.path).unwrap();
        serde_json::from_str(&data).unwrap_or_else(|_| vec![])
    }
}
