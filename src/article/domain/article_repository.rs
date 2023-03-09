use async_trait::async_trait;

#[async_trait]
pub trait ArticleRepository {
    async fn save(id: String, nombre: String, familia: String);
    async fn calc_id(family_id: String) -> anyhow::Result<String>;
    // fn find_by_id(id: String) -> Option<Self>;
    // fn find_by_subname(name: String) -> Option<Vec<Self>>;
    // fn update_name(id: String, new_name: String) -> anyhow::Result<()>;
    // fn update_family(id: String, new_family: String) -> anyhow::Result<()>;
    // fn remove(id: String) -> anyhow::Result<()>;
}
