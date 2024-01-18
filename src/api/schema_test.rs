use crate::{api, database};

#[test]
fn test_user_schema_from_user() {
    let user = database::model::User {
        id: "id".to_string(),
        name: "name".to_string(),
        email: "email".to_string(),
    };
    let user_schema = api::schema::UserSchema::from(user);
    assert_eq!(user_schema.id, "id");
    assert_eq!(user_schema.name, "name");
    assert_eq!(user_schema.email, "email");
}
