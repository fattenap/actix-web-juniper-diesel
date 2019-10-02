//! Actix web juniper diesel example

use std::io;
use std::sync::Arc;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
use actix_cors::Cors;
use actix_web::{http::header, middleware, web, App, Error, HttpResponse, HttpServer};
use actix_files as fs;

use dotenv::dotenv;
use futures::future::Future;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

mod db;
mod graphql;
mod models;
mod schema;
mod services;

use crate::db::establish_connection;
use crate::graphql::{create_schema, Context, Schema};

fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:8080/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

fn graphql(
    st: web::Data<Arc<Schema>>,
    ctx: web::Data<Context>,
    data: web::Json<GraphQLRequest>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let res = data.execute(&st, &ctx);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .map_err(Error::from)
    .and_then(|resp| {
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(resp))
    })
}

fn main() -> io::Result<()> {
    dotenv().ok();

    ::std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    let sys = actix::System::new("juniper-diesel-example");
    let pool = establish_connection();
    let schema_context = Context { db: pool.clone() };
    let schema = std::sync::Arc::new(create_schema());
    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .data(schema_context.clone())
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::new()
                    .allowed_origin("http://localhost:8080")
                    .allowed_methods(vec!["POST"])
                    .allowed_headers(vec![
                        header::AUTHORIZATION,
                        header::ACCEPT,
                        header::CONTENT_TYPE,
                    ])
                    .supports_credentials()
                    .max_age(3600),
            )
            .service(web::resource("/graphql").route(web::post().to_async(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
            .service(
                // static files
                fs::Files::new("/", "./dist/").index_file("index.html"),
            )
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .start();

    info!("Started http server: 127.0.0.1:8080");
    sys.run()
}
