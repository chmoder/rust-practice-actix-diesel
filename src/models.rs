use super::schema::posts;
use super::schema::zip_codes;
use super::schema::med_supp;
use serde::Serialize;
use serde::Deserialize;
use diesel::deserialize::QueryableByName;

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Queryable, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable, Deserialize)]
#[table_name="zip_codes"]
pub struct NewZipCode {
    pub zip_lookup_code: String,
    pub state: String,
    pub county: String,
    pub city: String,
    pub zip_3: String,
    pub zip_5: String,
}

#[derive(Queryable, Serialize)]
pub struct ZipCode {
    pub id: i32,
    pub zip_lookup_code: String,
    pub state: String,
    pub county: String,
    pub city: String,
    pub zip_3: String,
    pub zip_5: String,
}

#[derive(Insertable, Deserialize)]
#[table_name="med_supp"]
pub struct NewMedSupp {
    pub company: String,
    pub company_old: String,
    pub naic: String,
    pub plan: String,
    pub state: String,
    pub area: String,
    pub zip_lookup_code: String,
    pub gender: String,
    pub t_nt: String,
    pub couple_fac: String,
    pub eff_date: Option<chrono::NaiveDateTime>,
    pub rate_type: String,
    pub age_for_sorting: String,
    pub age: String,
    pub policy_fee: Option<String>,
    pub household_discount: Option<String>,
    pub monthly_rate: String,
    pub quarterly_rate: String,
    pub semi_annual_rate: String,
    pub annual_rate: String,
}

#[derive(QueryableByName, Queryable, Serialize)]
#[table_name="med_supp"]
pub struct MedSupp {
    pub id: i32,
    pub company: String,
    pub company_old: String,
    pub naic: String,
    pub plan: String,
    pub state: String,
    pub area: String,
    pub zip_lookup_code: String,
    pub gender: String,
    pub t_nt: String,
    pub couple_fac: String,
    pub eff_date: Option<chrono::NaiveDateTime>,
    pub rate_type: String,
    pub age_for_sorting: String,
    pub age: String,
    pub policy_fee: Option<String>,
    pub household_discount: Option<String>,
    pub monthly_rate: String,
    pub quarterly_rate: String,
    pub semi_annual_rate: String,
    pub annual_rate: String,
}