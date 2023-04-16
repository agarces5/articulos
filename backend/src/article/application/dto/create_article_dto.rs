#[derive(Debug, Clone)]
pub struct CreateArticleInputDTO {
    name: String,
    family_id: String,
}

impl CreateArticleInputDTO {
    pub fn new(name: &str, family_id: &str) -> Self {
        Self {
            name: name.to_string(),
            family_id: family_id.to_string(),
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn family_id(&self) -> &str {
        &self.family_id
    }
}
