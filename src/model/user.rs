use serde::Deserialize;

#[derive(Deserialize)]
pub struct User {
    name: String,
    password: String
}