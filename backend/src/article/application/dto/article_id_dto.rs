#[derive(Debug, Clone)]
pub struct ArticleIdInputDTO {
    id: i32,
}

impl ArticleIdInputDTO {
    pub fn new(id: i32) -> Self {
        Self { id }
    }
    pub fn id(&self) -> i32 {
        self.id
    }
}
