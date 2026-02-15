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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use crate::core::medication::Medication;

    fn temp_repo() -> JsonMedicationRepository {
        let file = NamedTempFile::new().unwrap();
        JsonMedicationRepository::new(file.path().to_str().unwrap())
    }

    #[test]
    fn test_save_and_get() {
        let mut repo = temp_repo();
        let med = Medication::new("Ibuprofen".to_string(), "Ibuprofen".to_string(), "200mg".to_string());
        repo.save(med.clone());
        let fetched = repo.get("Ibuprofen").unwrap();
        assert_eq!(fetched.name, "Ibuprofen");
        assert_eq!(fetched.substance, "Ibuprofen");
        assert_eq!(fetched.dose, "200mg");
    }

    #[test]
    fn test_list() {
        let mut repo = temp_repo();
        let med1 = Medication::new("A".to_string(), "X".to_string(), "1mg".to_string());
        let med2 = Medication::new("B".to_string(), "Y".to_string(), "2mg".to_string());
        repo.save(med1);
        repo.save(med2);
        let list = repo.list();
        assert_eq!(list.len(), 2);
    }
}
