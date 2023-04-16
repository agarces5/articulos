#[derive(Debug, Clone)]
pub struct UpdateArticleInputDTO {
    id: i32,
    name: Option<String>,
    family_id: Option<String>,
}

impl UpdateArticleInputDTO {
    pub fn new(id: i32, name: Option<String>, family_id: Option<String>) -> Self {
        Self {
            id,
            name,
            family_id,
        }
    }
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }
    pub fn family_id(&self) -> Option<String> {
        self.family_id.clone()
    }
}
