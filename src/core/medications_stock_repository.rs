/// MedicationsStockRepository defines the interface for stock persistence operations.
pub trait MedicationsStockRepository {
    /// Save the stock state to the repository.
    fn save(&mut self, stock: crate::medications_stock::MedicationsStock);

    /// Retrieve the stock state.
    fn get(&self) -> Option<crate::medications_stock::MedicationsStock>;
}
