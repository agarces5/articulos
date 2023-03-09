#[derive(Debug, Clone)]
pub struct Article {
    id: String,
    name: String,
    family_id: String,
}

impl Article {
    pub fn new(id: String, name: String, family_id: String) -> Self {
        Self {
            id,
            name,
            family_id,
        }
    }
    // pub fn id(&self) -> &str {
    //     &self.id
    // }
    // pub fn nombre(&self) -> &str {
    //     &self.nombre
    // }
    // pub fn familia(&self) -> &str {
    //     &self.familia
    // }
}

impl ToString for Article {
    fn to_string(&self) -> String {
        format!(
            "Articulo: {},\t Nombre: {},\t Familia: {})",
            self.id, self.name, self.family_id
        )
    }
}
