#[derive(Debug)]
pub struct Articulo {
    id: String,
    nombre: String,
    familia: String,
}

impl Articulo {
    pub fn new(id: String, nombre: String, familia: String) -> Self {
        Self {
            id,
            nombre,
            familia,
        }
    }
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn nombre(&self) -> &str {
        &self.nombre
    }
    pub fn familia(&self) -> &str {
        &self.familia
    }
}

impl ToString for Articulo {
    fn to_string(&self) -> String {
        format!(
            "Articulo: {},\t Nombre: {},\t Familia: {})",
            self.id, self.nombre, self.familia
        )
    }
}
