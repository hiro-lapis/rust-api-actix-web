use async_graphql::InputObject;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))] // テーブル定義と合致するかチェック
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

#[derive(InputObject, Debug)] // graphqlのInput型として公開
pub struct CreateUserInput {
    // inputの値は全て公開する
    pub name: String, // use dslがある箇所で定義するとlet bindings cannot shadow エラーが発生するので注意
    pub email: String,
    pub password: String,
}

// SimpleObjectを使うと構造体を自動でgraphql getter付きの構造体定義にしてくれる(フィールドはcamel caseにリネーム)
// https://docs.rs/async-graphql/latest/async_graphql/derive.SimpleObject.html
#[derive(async_graphql::SimpleObject)]
pub struct Category {
    pub id: i32,
    pub name: String,
    // Datetime型を扱うためにはasync_graphqlのfeature.chronoを入れること
    pub created_at: chrono::NaiveDate,
    pub sub_categories: Vec<SubCategory>,
}

#[derive(async_graphql::SimpleObject)]
#[graphql(name = "Renamed")] // 構造体をgraphql schemaとして公開するときの名前をリネーム可能
pub struct SubCategory {
    #[graphql(skip)] // graphql schemaとして公開しないようにできる
    pub id: i32,
    pub name: String,
}
