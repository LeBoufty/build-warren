use crate::build_parser::fetch_build_order;
use crate::handlers::{fetch_latest, fetch_segment};
use crate::index_manager::get_st_highest_index;
use actix_web::web::Json;
use actix_web::{App, HttpServer, Responder, get, web};

#[get("/build/{id}")]
async fn get_by_id(name: web::Path<u32>) -> impl Responder {
    Json(fetch_build_order(name.into_inner()))
}

#[get("/latest/{count}")]
async fn get_latest(count: web::Path<u32>) -> impl Responder {
    Json(fetch_latest(count.into_inner()))
}

#[get("/latest")]
async fn get_latest_default() -> impl Responder {
    Json(fetch_latest(1)) // Default to 1 if no count is provided
}

#[get("/segment/{start}/{end}")]
async fn get_segment(segment: web::Path<(u32, u32)>) -> impl Responder {
    let (start, end) = segment.into_inner();
    Json(fetch_segment(start, end))
}

#[get("/count")]
async fn get_count() -> impl Responder {
    Json(get_st_highest_index())
}

#[actix_web::main]
pub async fn run(port: u16) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_by_id)
            .service(get_latest)
            .service(get_latest_default)
            .service(get_segment)
            .service(get_count)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
