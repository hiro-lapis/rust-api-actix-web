use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{App, HttpResponse, HttpServer, Result, guard, web};
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use dotenvy::dotenv;
use std::env;
use std::net::{Ipv4Addr, Ipv6Addr};

use app::{ApiSchema, build_schema};

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
    let schema = build_schema();

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
