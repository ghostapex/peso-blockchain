use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValidatorIdentity {
    pub id: String,
    pub public_key: String,
    pub commission: u64,
}

pub struct ValidatorRegistry {
    pub validators: Vec<ValidatorIdentity>,
}

impl ValidatorRegistry {
    pub fn new() -> Self {
        Self {
            validators: Vec::new(),
        }
    }

    pub fn register(&mut self, id: String, public_key: String, commission: u64) {
        self.validators.push(ValidatorIdentity {
            id,
            public_key,
            commission,
        });
    }

    pub fn get(&self, id: &str) -> Option<&ValidatorIdentity> {
        self.validators.iter().find(|v| v.id == id)
    }
}
