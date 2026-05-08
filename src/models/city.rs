use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug, serde::Serialize)]
#[diesel(table_name = crate::schema::city)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct City {
    pub city_id: i32,
    pub city_col: String, // Corresponds to #[sql_name = "city"] in schema
    pub country_id: i16,
    pub last_update: chrono::NaiveDateTime,
}
