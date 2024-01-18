#[derive(sqlx::FromRow, serde::Serialize, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
}
