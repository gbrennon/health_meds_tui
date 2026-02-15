/// Represents a medication with its name, substance, and dose.
#[derive(Clone, Debug, PartialEq)]
pub struct Medication {
    pub name: String,
    pub substance: String,
    pub dose: String,
}

impl Medication {
    /// Creates a new Medication instance.
    pub fn new(name: String, substance: String, dose: String) -> Self {
        Self { name, substance, dose }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_medication_new() {
        let med = Medication::new("Aspirin".to_string(), "Acetylsalicylic Acid".to_string(), "100mg".to_string());
        assert_eq!(med.name, "Aspirin");
        assert_eq!(med.substance, "Acetylsalicylic Acid");
        assert_eq!(med.dose, "100mg");
    }
}
