use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(
    Debug,
    Queryable,
    Selectable,
)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// Userという型をGraphQLで返すために、async_graphql::Objectをimplする
// TODO: SimpleObjectを使うと、デフォルトでSerializeされるかも？試す
#[async_graphql::Object]
impl User {
    pub async fn id(&self) -> i32 {
        self.id
    }

    pub async fn name(&self) -> &str {
        &self.name
    }

    pub async fn email(&self) -> &str {
        &self.email
    }

    pub async fn password(&self) -> &str {
        &self.password
    }

    pub async fn created_at(&self) -> String {
        self.created_at.to_string()
    }

    pub async fn updated_at(&self) -> String {
        self.updated_at.to_string()
    }
}
