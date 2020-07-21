pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use self::models::{Post, NewPost};
use crate::models::{ZipCode, NewZipCode, NewMedSupp, MedSupp};
use diesel::sql_query;
use actix_web::http::header::q;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


pub fn create_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Post {
    use schema::posts;

    let new_post = NewPost {
        title: title,
        body: body,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn get_posts(conn: &PgConnection) -> std::vec::Vec<Post> {
    use self::schema::posts::dsl::*;

    let results = posts.filter(published.eq(false))
        .limit(5)
        .load::<Post>(conn)
        .expect("Error loading posts");

    results
}

pub fn create_zip_code<'a>(
    conn: &PgConnection,
    new_zip_code: &NewZipCode,
) -> ZipCode {
    use schema::zip_codes;

    diesel::insert_into(zip_codes::table)
        .values(new_zip_code)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn create_med_supp<'a>(
    conn: &PgConnection,
    new_med_supp: &NewMedSupp,
) -> MedSupp {
    use schema::med_supp;

    diesel::insert_into(med_supp::table)
        .values(new_med_supp)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn get_zip_lookup_codes(conn: &PgConnection, zip5: String) -> std::vec::Vec<std::string::String> {
    use schema::zip_codes::dsl::*;

    zip_codes
        .select(zip_lookup_code)
        .distinct()
        .filter(zip_5.eq(zip5))
        .load::<String>(conn)
        .expect("error loading area")
}

fn build_search_med_supp_query(
    zip5: String,
    plan: String,
    age: String,
    gender: String,
    tobacco: String,
) -> String {
    format!("select
                    med_supp.*
                from
                    med_supp
                inner join zip_codes
                    on med_supp.zip_lookup_code = zip_codes.zip_lookup_code
                where
                    zip_codes.zip_5 = '{zip5}' and
                    med_supp.eff_date > '2018-01-01'and
                    med_supp.age = '{age}' and
                    med_supp.gender = '{gender}' and
                    med_supp.plan = '{plan}' and
                    med_supp.t_nt = '{tobacco}'
    ", zip5=zip5, age=age, gender=gender, plan=plan, tobacco=tobacco)
}

pub fn search_med_supp(
    conn: &PgConnection,
    zip5_: String,
    plan_: String,
    age_: String,
    gender_: String,
    tobacco_: String,
) -> Vec<MedSupp>{
    use schema::med_supp::dsl::*;

    let query_string = build_search_med_supp_query(
        zip5_,
        plan_,
        age_,
        gender_,
        tobacco_
    );


    // ^ THIS IS A TEXT BOOK EXAMPLE OF A SQL INJECTION VULNERABILITY ^
   sql_query(query_string).load::<MedSupp>(conn).unwrap()

    // let zip_code_areas = get_zip_lookup_codes(zip5);
    //
    // let data = med_supp
    //     // .inner_join(zip_codes::table.zip_lookup_code.on(med_supp::table.zip_lookup_code))
    //     .filter(zip_code_area.in())
    //     .filter(age.eq("65"))
    //     .load::<MedSupp>(conn)
    //     .expect("error loading med_supps");
}
