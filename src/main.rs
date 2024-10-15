use actix_web::{get, web, App, HttpServer, Responder};
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use leptos::*;
use leptos_actix::{generate_route_list, LeptosRoutes};
mod frontend;

async fn connect_db() -> Surreal<Ws> {
    let db = Surreal::new::<Ws>("ws://127.0.0.1:8000").await.unwrap();
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .unwrap();
    db.use_ns("ns1").use_db("db1").await.unwrap();
    db
}

#[get("/")]
async fn index() -> impl Responder {
    "Hello, Leptos + Actix + SurrealDB!"
}

#[get("/surveys/1")]
async fn get_user(db: web::Data<Surreal<Ws>>, survey_id: web::Path<String>) -> impl Responder {
    let res = db.select(("surveys", &*survey_id)).await.unwrap();
    web::Json(res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = connect_db().await;

    HttpServer::new(move || {
        let leptos_routes = generate_route_list(|cx| view! { cx, <frontend::App/> });
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(LeptosRoutes::new("/", leptos_routes))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}