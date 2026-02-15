/// MedicationRepository defines the interface for medication persistence operations.
pub trait MedicationRepository {
    /// Save a medication to the repository.
    fn save(&mut self, medication: crate::medication::Medication);

    /// Retrieve a medication by name.
    fn get(&self, name: &str) -> Option<crate::medication::Medication>;

    /// List all medications in the repository.
    fn list(&self) -> Vec<crate::medication::Medication>;
}
