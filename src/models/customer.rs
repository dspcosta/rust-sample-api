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

#[derive(Insertable, serde::Deserialize)]
#[diesel(table_name = crate::schema::customer)]
pub struct NewCustomer {
    pub store_id: i16,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub address_id: i16,
    pub activebool: bool,
    pub active: Option<i32>,
}

#[derive(AsChangeset, serde::Deserialize)]
#[diesel(table_name = crate::schema::customer)]
pub struct UpdateCustomer {
    pub store_id: i16,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub address_id: i16,
    pub activebool: bool,
    pub active: Option<i32>,
}
