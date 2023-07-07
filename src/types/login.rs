use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LoginPayload {
    pub username: String,
    pub pwd: String,
}
