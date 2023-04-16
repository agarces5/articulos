use tiberius::numeric::Numeric;

use crate::article::domain::article::Article;

#[derive(Debug, Clone, PartialEq)]
pub struct TiberiusArticle {
    id: Numeric,
    name: String,
    family_id: String,
    user: String,
}

impl TiberiusArticle {
    pub fn new(id: Numeric, name: &str, family_id: &str, user: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            family_id: family_id.to_string(),
            user: user.to_string(),
        }
    }

    pub fn id(&self) -> Numeric {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn family_id(&self) -> &str {
        &self.family_id
    }
    pub fn user(&self) -> &str {
        &self.user
    }
    pub fn set_name(&mut self, new_name: &str) {
        self.name = new_name.to_string()
    }
    pub fn set_family_id(&mut self, new_family: &str) {
        self.family_id = new_family.to_string()
    }
}
impl From<&Article> for TiberiusArticle {
    fn from(article: &Article) -> Self {
        Self {
            id: Numeric::new_with_scale(article.id() as i128, 0),
            name: article.name().to_string(),
            family_id: article.family_id().to_string(),
            user: article.user().to_string(),
        }
    }
}
// impl Into<Article> for TiberiusArticle {
//     fn into(self) -> Article {
//         Article::new(
//             self.id().int_part() as i32,
//             self.name(),
//             self.family_id(),
//             self.user(),
//         )
//     }
// }

impl From<TiberiusArticle> for Article {
    fn from(tiberius_article: TiberiusArticle) -> Self {
        Article::new(
            tiberius_article.id().int_part() as i32,
            tiberius_article.name(),
            tiberius_article.family_id(),
            tiberius_article.user(),
        )
    }
}
