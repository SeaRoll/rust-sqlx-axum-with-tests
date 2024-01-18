pub const CREATE_USER: &str = r#"
    INSERT INTO "user" (id, name, email)
    VALUES ($1, $2, $3)
"#;

pub const GET_USERS: &str = r#"
    SELECT id, name, email
    FROM "user"
"#;
