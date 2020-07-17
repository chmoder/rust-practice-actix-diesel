use actix_web::{web, get, post, App, Responder, HttpServer, error, HttpRequest, HttpResponse};
use deadpool_redis::{Config, Pool};
use deadpool_redis::redis::AsyncCommands;
use rust_practice_actix_web::{establish_connection, get_posts, create_post, create_zip_code, create_med_supp, search_med_supp};
use diesel::PgConnection;
use actix_web::web::Json;
use rust_practice_actix_web::models::{NewZipCode, NewMedSupp, MedSupp, ZipCode};
use chrono::NaiveDateTime;

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


async fn get_from_redis(key: String, redis_pool: web::Data<Pool>) -> String {
    let mut conn = redis_pool.get().await.unwrap();
    return conn.get(key).await.unwrap_or_default();
}

async fn add_to_redis(key: String, val: String, redis_pool: web::Data<Pool>) {
    let mut conn = redis_pool.get().await.unwrap();
    let _:() = conn.set(key, val).await.unwrap_or_default();
}

#[get("/{id}/{name}/index.html")]
async fn index(path_params: web::Path<(String, u32)>, redis_pool: web::Data<Pool>, pg_connection: web::Data<PgConnection>) -> impl Responder {
    // let mut item= get_from_redis(path_params.0.clone(), redis_pool.clone()).await;
    //
    // if item.is_empty() {
    //     add_to_redis(path_params.0.clone(), "42".to_string(), redis_pool.clone()).await;
    //     item = get_from_redis(path_params.0.clone(), redis_pool.clone()).await;
    // }

    create_post(&pg_connection, "post title 1", "post body 1");
    let posts = get_posts(&pg_connection);
    Json(posts)

    // Ok(Json(posts))
    // format!("Hello {}! id:{} redis_value:{}", path_params.0, path_params.1, item)
}

#[post("/zip-codes")]
async fn create_zip_code_handler(new_item: web::Json<NewZipCode>, redis_pool: web::Data<Pool>, pg_connection: web::Data<PgConnection>) -> impl Responder {
    let zip_code = create_zip_code(&pg_connection, &new_item);
    Json(zip_code)
}

#[post("/med-supp")]
async fn create_med_supp_handler(new_item: web::Json<NewMedSupp>, redis_pool: web::Data<Pool>, pg_connection: web::Data<PgConnection>) -> impl Responder {
    let med_supp = create_med_supp(&pg_connection, &new_item);
    Json(med_supp)
}

use serde::Deserialize;
#[derive(Deserialize)]
pub struct MedSuppQuoteRequest {
    zip5: String,
    age: String,
    gender: String,
    tobacco: String,
    plan: String,
}

#[get("/med-supp")]
async fn med_supp_quote_list(web::Query(info): web::Query<MedSuppQuoteRequest>, redis_pool: web::Data<Pool>, pg_connection: web::Data<PgConnection>) -> impl Responder {

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
    let cfg = Config::from_env("prefix").unwrap();
    let pool = cfg.create_pool().unwrap();

    let pg_connection = establish_connection();

    HttpServer::new(move || App::new()
        .data(pool.clone())
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
        .bind("0.0.0.0:8081")?
        .run()
        .await
}