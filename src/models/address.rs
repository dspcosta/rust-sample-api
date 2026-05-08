use diesel::prelude::*;

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
