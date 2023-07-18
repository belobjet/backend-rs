use actix_cors::Cors;
use actix_web::{
    get, middleware, route,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use std::{io, sync::Arc};

use actix_web_lab::respond::Html;
use firebase_auth::{FirebaseAuth, FirebaseUser};
use juniper::http::{playground::playground_source, GraphQLRequest};

mod schema;
use crate::schema::{create_schema, Database, Schema};

mod firestore;
use crate::firestore::get_client;

mod loaders;
mod models;

#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html(playground_source("/graphql", None))
}

#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(
    st: web::Data<Schema>,
    user: FirebaseUser,
    data: web::Json<GraphQLRequest>,
) -> impl Responder {
    log::info!("auth: {:?}", user.email);

    let db = match get_client().await {
        Ok(db) => db,
        Err(err) => {
            log::error!("Error getting firestore client: {}", err);
            return HttpResponse::InternalServerError().body("Error getting firestore client");
        }
    };

    let profile_loader = loaders::get_loader(db.clone());
    let context = Database::new(db, profile_loader);
    let user = data.execute(&st, &context).await;
    HttpResponse::Ok().json(user)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let firebase_auth = tokio::task::spawn_blocking(|| FirebaseAuth::new("mystvff-a9ad3"))
        .await
        .expect("panic init FirebaseAuth");

    let schema = Arc::new(create_schema());
    let auth = Data::new(firebase_auth);
    log::info!("Started http on port 8080");
    log::info!("GraphQL playground at http://localhost:8080/graphiql");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(Data::from(schema.clone()))
            .app_data(auth.clone())
            .service(graphql)
            .service(graphql_playground)
            // the graphiql UI requires CORS to be enabled
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
