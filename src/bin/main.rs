use actix_web::web::Data;
use actix_web::{guard, web, App, HttpResponse, HttpServer, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use chrono::{Datelike, FixedOffset, Timelike, Utc};
use std::net::Ipv4Addr;

struct Query;

#[Object]
impl Query {
    // # on the frontend { totalPhotos }
    async fn total_photos(&self) -> usize {
        42
    }
    async fn now(&self) -> String {
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
}

type ApiSchema = Schema<Query, EmptyMutation, EmptySubscription>;

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
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();

    println!("Playground: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            // json api
            .service(web::resource("/").guard(guard::Post()).to(index))
            // html api
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
    .bind((Ipv4Addr::LOCALHOST, 8080))?
    .run()
    .await
}
