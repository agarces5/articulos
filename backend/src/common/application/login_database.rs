use crate::common::{
    application::dto::login_input_dto::LoginInputDTO, domain::db_connection::DBConnection,
};

pub struct LoginDatabase {
    database: Box<dyn DBConnection>,
}

impl LoginDatabase {
    pub fn new(database: Box<dyn DBConnection>) -> Self {
        Self { database }
    }

    pub async fn execute(&mut self, input: LoginInputDTO) -> bool {
        let (username, password) = (&input.username(), &input.password());
        // Conectar a BBDD
        let client = self.database.set_tcp_client(username, password, "").await;
        client.is_ok()
    }
}
