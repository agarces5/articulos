use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LoginOutputDTO {
    is_logged: bool,
}

impl LoginOutputDTO {
    pub fn new(is_logged: bool) -> Self {
        Self { is_logged }
    }
    pub fn is_logged(&self) -> bool {
        self.is_logged
    }
}
