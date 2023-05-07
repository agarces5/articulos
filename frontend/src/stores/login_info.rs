use serde::{Deserialize, Serialize};
use yewdux::store::Store;

#[derive(Debug, Default, Store, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct LoginInfo {
    pub username: Option<String>,
    pub password: Option<String>,
    pub is_logged: bool,
}
