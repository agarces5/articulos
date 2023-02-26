use std::fmt::Display;

#[derive(Debug)]
pub struct Articulo {
    pub articulo: String,
    pub nombre: String,
    pub familia: String,
}

impl Display for Articulo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Articulo: {},\t Nombre: {},\t Familia: {})",
            self.articulo, self.nombre, self.familia
        )
    }
}
