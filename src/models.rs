use chrono::NaiveDate;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug, serde::Serialize)]
#[diesel(table_name = crate::schema::customer)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Customer {
    pub customer_id: i32,
    pub store_id: i16,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub address_id: i16,
    pub activebool: bool,
    pub create_date: NaiveDate,
    pub last_update: Option<chrono::NaiveDateTime>,
    pub active: Option<i32>,
}

#[derive(Queryable, Selectable, Debug, serde::Serialize)]
#[diesel(table_name = crate::schema::address)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Address {
    pub address_id: i32,
    pub address_col: String, // Corresponds to #[sql_name = "address"] in schema
    pub address2: Option<String>,
    pub district: String,
    pub city_id: i16,
    pub postal_code: Option<String>,
    pub phone: String,
    pub last_update: chrono::NaiveDateTime,
}

#[derive(Queryable, Selectable, Debug, serde::Serialize)]
#[diesel(table_name = crate::schema::city)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct City {
    pub city_id: i32,
    pub city_col: String, // Corresponds to #[sql_name = "city"] in schema
    pub country_id: i16,
    pub last_update: chrono::NaiveDateTime,
}
