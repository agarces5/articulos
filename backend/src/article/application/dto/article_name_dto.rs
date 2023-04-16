#[derive(Debug, Clone)]
pub struct ArticleNameInputDTO {
    subname: String,
}

impl ArticleNameInputDTO {
    pub fn new(subname: &str) -> Self {
        Self {
            subname: subname.to_string(),
        }
    }
    pub fn subname(&self) -> &str {
        &self.subname
    }
}
