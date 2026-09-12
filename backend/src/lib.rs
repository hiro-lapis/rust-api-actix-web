use async_graphql::{EmptySubscription, Object, Result as GqlResult, Schema};
use chrono::{Datelike, FixedOffset, Timelike, Utc};
use db_schema::model::*;
use db_schema::schema::users::dsl::*;
use diesel::{insert_into, prelude::*};
use dotenvy::dotenv;
use std::env;

// NOTE: this file cannot put in src/lib because of convention over
// https://doc.rust-lang.org/cargo/guide/project-layout.html#package-layout

// Query, Mutationについて
// https://async-graphql.github.io/async-graphql/en/query_and_mutation.html#query-root-object
pub struct Query;

#[Object]
impl Query {
    // # on the frontend, query { totalPhotos }
    async fn total_photos(&self) -> usize {
        42
    }

    // # on the frontend, query { now }
    async fn now(&self) -> String {
        // Queryを実装するクラスが&selfで実行する
        let jp_now = Utc::now().with_timezone(FixedOffset::east_opt(9 * 3600).as_ref().unwrap());
        format!(
            "{}年{}月{}日 {}:{}:{}",
            jp_now.year(),
            jp_now.month(),
            jp_now.day(),
            jp_now.hour(),
            jp_now.minute(),
            jp_now.second()
        )
    }
    async fn users(&self) -> Vec<User> {
        let con = &mut establish_connection();
        let res = users.select(User::as_select()).load(con);
        if let Ok(u) = res {
            dbg!(&u);
            vec![User {
                id: 1,
                name: "John".to_string(),
                email: "john@example.com".to_string(),
                password: "password".to_string(),
                created_at: chrono::Utc::now().naive_utc(),
                updated_at: chrono::Utc::now().naive_utc(),
            }]
            // vec![]
        } else {
            vec![]
        }
    }

    async fn categories(&self) -> Vec<Category> {
        vec![
            Category {
                id: 1,
                name: "novel".to_string(),
                created_at: chrono::NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
                sub_categories: vec![SubCategory {
                    id: 10,
                    name: "right novel".to_string(),
                }],
            },
            Category {
                id: 2,
                name: "magazine".to_string(),
                created_at: chrono::NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
                sub_categories: vec![SubCategory {
                    id: 20,
                    name: "weekly magazine".to_string(),
                }],
            },
        ]
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    /// create user(TODO: hash password)
    /// example:
    ///
    /// mutation {
    ///     createUser(input: { name: "hanako", email: "hanako@example.com", password: "pass5678" })
    ///   }
    ///
    async fn create_user(&self, input: CreateUserInput) -> GqlResult<bool> {
        dbg!(&input);
        let con = &mut establish_connection();
        let res = insert_into(users)
            .values(
                (
                    name.eq(input.name),
                    email.eq(input.email),
                    password.eq(input.password),
                ), // 取得が不要な場合はexecute
            )
            .execute(con)
            // Graphqlサーバにおいてはエラー発生時の専用のエラーレスポンスを返す必要がある
            // dieselのDBエラー型をそのまま返却するのはimpl Queryの制約に反するので型を変換してあげる
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(res > 0)
    }

    /// create users(TODO: hash password)
    /// example:
    ///
    /// mutation {
    ///     createUsers(inputs: [
    ///         { name: "hanako", email: "hanako_multi@example.com", password: "pass5678" },
    ///         { name: "jiro", email: "jiro_multi@example.com", password: "pass1234" }
    ///     ]) {
    ///         id
    ///         name
    ///         email
    ///         password
    ///         created_at
    ///         updated_at
    ///     }
    /// }
    async fn create_users(&self, inputs: Vec<CreateUserInput>) -> GqlResult<Vec<User>> {
        let con = &mut establish_connection();
        let res = insert_into(users)
            .values(
                inputs
                    .iter()
                    .map(|input| {
                        (
                            name.eq(input.name.clone()),
                            email.eq(input.email.clone()),
                            password.eq(input.password.clone()),
                        )
                    })
                    .collect::<Vec<_>>(),
            )
            .get_results(con)
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(res)
    }
}

// 実装初期にSchemaを初期化したい時はEmptyMutationを使用する
// type ApiSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub type ApiSchema = Schema<Query, Mutation, EmptySubscription>;

/// build graphql schema for api
pub fn build_schema() -> ApiSchema {
    // ここでGraphqサーバとしてのスキーマ定義をバインドする。
    // ここにMutationを使うようにしてもApiSchemaの定義でEmptyMutationを使用してるとMutationが使えないので注意
    Schema::build(Query, Mutation, EmptySubscription).finish()
}

/// connect postgresql and return connection
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("database_url: {}", database_url);
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// フロントエンドは起動中サーバの introspection から型を生成するため、
    /// スキーマに公開されるフィールドが意図せず消えていないことを確認する
    #[test]
    fn schema_exposes_dashboard_fields() {
        let sdl = build_schema().sdl();
        assert!(sdl.contains("now: String!"), "sdl: {}", sdl);
        assert!(sdl.contains("totalPhotos: Int!"), "sdl: {}", sdl);
    }
}
