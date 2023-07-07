pub mod login;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Foo {
    pub uid: u64,
    pub uname: String,
}

#[derive(Debug, Deserialize)]
pub struct HelloParams {
    pub name: Option<String>,
}
