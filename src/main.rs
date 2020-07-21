use actix_web::{web, get, post, App, Responder, HttpServer, error, HttpRequest, HttpResponse};
use deadpool_redis::{Config, Pool};
use deadpool_redis::redis::AsyncCommands;
use diesel::PgConnection;
use actix_web::web::Json;
use chrono::NaiveDateTime;
use serde::Deserialize;
use rust_practice_actix_diesel::models::{NewZipCode, NewMedSupp};
use rust_practice_actix_diesel::{create_post, get_posts, create_zip_code, create_med_supp, search_med_supp, establish_connection};


fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
    let detail = err.to_string();
    let response = match &err {
        error::JsonPayloadError::ContentType => {
            HttpResponse::UnsupportedMediaType().content_type("text/plain").body(detail)
        }
        _ => HttpResponse::BadRequest().content_type("text/plain").body(detail),
    };
    error::InternalError::from_response(err, response).into()
}

#[get("/{id}/{name}/index.html")]
async fn index(path_params: web::Path<(String, u32)>, pg_connection: web::Data<PgConnection>) -> impl Responder {
    create_post(&pg_connection, "post title 1", "post body 1");
    let posts = get_posts(&pg_connection);
    Json(posts)
}

#[post("/zip-codes")]
async fn create_zip_code_handler(new_item: web::Json<NewZipCode>, pg_connection: web::Data<PgConnection>) -> impl Responder {
    let zip_code = create_zip_code(&pg_connection, &new_item);
    Json(zip_code)
}

#[post("/med-supp")]
async fn create_med_supp_handler(new_item: web::Json<NewMedSupp>, pg_connection: web::Data<PgConnection>) -> impl Responder {
    let med_supp = create_med_supp(&pg_connection, &new_item);
    Json(med_supp)
}

#[derive(Deserialize)]
pub struct MedSuppQuoteRequest {
    zip5: String,
    age: String,
    gender: String,
    tobacco: String,
    plan: String,
}

#[get("/med-supp")]
async fn med_supp_quote_list(web::Query(info): web::Query<MedSuppQuoteRequest>, pg_connection: web::Data<PgConnection>) -> impl Responder {

    let thing = search_med_supp(
        &pg_connection,
        info.zip5,
        info.plan,
        info.age,
        info.gender,
        info.tobacco,
    );

    Json(thing)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new()
        .data(establish_connection())
        .service(
        index
        ).service(
        create_zip_code_handler
        )
        .service(
            create_med_supp_handler
        )
        .service(
            med_supp_quote_list
        )
    )
        .workers(num_cpus::get())
        .bind("0.0.0.0:8080")?
        .run()
        .await
}