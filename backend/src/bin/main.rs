use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{App, HttpResponse, HttpServer, Result, guard, web};
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql::{EmptySubscription, Object, Result as GqlResult, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use chrono::{Datelike, FixedOffset, Timelike, Utc};
use db_schema::model::*;
use db_schema::schema::users::dsl::*;
use diesel::{insert_into, prelude::*};
use dotenvy::dotenv;
use std::env;
use std::net::{Ipv4Addr, Ipv6Addr};

// Query, Mutationについて
// https://async-graphql.github.io/async-graphql/en/query_and_mutation.html#query-root-object
struct Query;

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

struct Mutation;

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

type ApiSchema = Schema<Query, Mutation, EmptySubscription>;

async fn index(schema: web::Data<ApiSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn index_playground() -> Result<HttpResponse> {
    let source = playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load env vars from `.env` if present (docker-compose / cargo-make may also provide env directly).
    dotenv().ok();
    // ここでGraphqサーバとしてのスキーマ定義をバインドする。
    // ここにMutationを使うようにしてもApiSchemaの定義でEmptyMutationを使用してるとMutationが使えないので注意
    let schema = Schema::build(Query, Mutation, EmptySubscription).finish();

    println!("Playground: http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin(
                        env::var("ALLOW_ORIGIN")
                            .expect("allow origin is not set")
                            .as_str(),
                    )
                    .allowed_methods(vec!["GET", "POST"])
                    .allow_any_header(),
            )
            .app_data(Data::new(schema.clone()))
            // json api
            .service(web::resource("/").guard(guard::Post()).to(index))
            // html api
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
    .bind((Ipv6Addr::LOCALHOST, 8080))? // localhost base
    .bind((Ipv4Addr::LOCALHOST, 8080))? // 127.0.0.1 base
    .run()
    .await
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("database_url: {}", database_url);
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
