use crate::core::medication::Medication;
use std::collections::HashMap;

/// Aggregate root for managing medication stock and registration.
pub struct MedicationsStock {
    pub stock: HashMap<String, u32>, // medication name -> amount
    pub medications: HashMap<String, Medication>, // medication name -> Medication
}

impl MedicationsStock {
    /// Static factory method to create a new MedicationsStock instance.
    pub fn new() -> Self {
        MedicationsStock {
            stock: HashMap::new(),
            medications: HashMap::new(),
        }
    }

    /// Adds a medication to the stock registry.
    pub fn add_medication(&mut self, med: Medication) {
        self.medications.insert(med.name.clone(), med.clone());
        self.stock.insert(med.name.clone(), 0);
    }

    /// Adds additional stock for a medication.
    pub fn add_stock(&mut self, med_name: &str, amount: u32) {
        if let Some(stock) = self.stock.get_mut(med_name) {
            *stock += amount;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::medication::Medication;

    #[test]
    fn test_new_and_add_medication() {
        let mut stock = MedicationsStock::new();
        let med = Medication::new("Aspirin".to_string(), "Acetylsalicylic Acid".to_string(), "100mg".to_string());
        stock.add_medication(med);
        assert!(stock.medications.contains_key("Aspirin"));
        assert_eq!(*stock.stock.get("Aspirin").unwrap(), 0);
    }

    #[test]
    fn test_add_stock() {
        let mut stock = MedicationsStock::new();
        let med = Medication::new("Aspirin".to_string(), "Acetylsalicylic Acid".to_string(), "100mg".to_string());
        stock.add_medication(med);
        stock.add_stock("Aspirin", 5);
        assert_eq!(*stock.stock.get("Aspirin").unwrap(), 5);
    }
}
