table! {
    med_supp (id) {
        id -> Int4,
        company -> Varchar,
        company_old -> Varchar,
        naic -> Varchar,
        plan -> Varchar,
        state -> Varchar,
        area -> Varchar,
        zip_lookup_code -> Varchar,
        gender -> Varchar,
        t_nt -> Varchar,
        couple_fac -> Varchar,
        eff_date -> Nullable<Timestamp>,
        rate_type -> Varchar,
        age_for_sorting -> Varchar,
        age -> Varchar,
        policy_fee -> Nullable<Varchar>,
        household_discount -> Nullable<Varchar>,
        monthly_rate -> Varchar,
        quarterly_rate -> Varchar,
        semi_annual_rate -> Varchar,
        annual_rate -> Varchar,
    }
}

table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

table! {
    zip_codes (id) {
        id -> Int4,
        zip_lookup_code -> Varchar,
        state -> Varchar,
        county -> Varchar,
        city -> Varchar,
        zip_3 -> Varchar,
        zip_5 -> Varchar,
    }
}

// joinable!(med_supp -> zip_codes (zip_lookup_code));

allow_tables_to_appear_in_same_query!(
    med_supp,
    posts,
    zip_codes,
);
