use crate::database;

#[derive(serde::Serialize)]
pub struct ErrorSchema {
    pub error: String,
    pub message: String,
    pub status: u16,
}

#[derive(serde::Serialize)]
pub struct EmptySuccessSchema {
    pub success: bool,
}

#[derive(serde::Deserialize)]
pub struct CreateUserSchema {
    pub name: String,
    pub email: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserSchema {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl From<database::model::User> for UserSchema {
    fn from(u: database::model::User) -> Self {
        UserSchema {
            id: u.id,
            name: u.name,
            email: u.email,
        }
    }
}
