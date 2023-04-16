use tauri::http::{Request, Response};

use crate::common::application::{
    dto::{login_input_dto::LoginInputDTO, login_output_dto::LoginOutputDTO},
    login_database::LoginDatabase,
};

pub struct LoginController {
    service: LoginDatabase,
}

impl LoginController {
    pub fn new(service: LoginDatabase) -> Self {
        Self { service }
    }
    pub async fn execute(&mut self, req: Request) -> Response {
        let input = serde_json::from_slice::<LoginInputDTO>(req.body()).unwrap_or_default();
        let resp = self.service.execute(input).await;
        let output = LoginOutputDTO::new(resp);
        Response::new(
            serde_json::to_value(output)
                .unwrap()
                .to_string()
                .into_bytes(),
        )
    }
}

#[cfg(test)]
mod tests {

    use serde_json::json;
    use tauri::http::status::StatusCode;

    use crate::common::infrastructure::db::tiberius::create_config::TiberiusClient;

    use super::*;

    #[tokio::test]
    async fn login_test() {
        let body = json!({
            "username": "SA",
            "password": "Sqlserver-2017",
        })
        .to_string();
        println!("{:?}", body.clone());
        // INPUT
        let req = Request::new(body.into_bytes());
        // Connection
        let conn = TiberiusClient::new();
        // USE CASE
        let login_use_case = LoginDatabase::new(Box::new(conn));
        // CONTROLLER
        let mut login_controller = LoginController::new(login_use_case);
        // ASSERTS
        let resp = login_controller.execute(req).await;
        assert_eq!(StatusCode::OK, resp.status());
        assert_eq!(
            String::from_utf8_lossy(
                Response::new(r#"{"is_logged":true}"#.as_bytes().to_vec()).body()
            ),
            String::from_utf8_lossy(resp.body())
        );
    }
}
