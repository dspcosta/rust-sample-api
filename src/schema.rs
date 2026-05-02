// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "mpaa_rating"))]
    pub struct MpaaRating;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "tsvector", schema = "pg_catalog"))]
    pub struct Tsvector;
}

diesel::table! {
    actor (actor_id) {
        actor_id -> Int4,
        #[max_length = 45]
        first_name -> Varchar,
        #[max_length = 45]
        last_name -> Varchar,
        last_update -> Timestamp,
    }
}

diesel::table! {
    address (address_id) {
        address_id -> Int4,
        #[max_length = 50]
        #[sql_name = "address"]
        address_col -> Varchar,
        #[max_length = 50]
        address2 -> Nullable<Varchar>,
        #[max_length = 20]
        district -> Varchar,
        city_id -> Int4,
        #[max_length = 10]
        postal_code -> Nullable<Varchar>,
        #[max_length = 20]
        phone -> Varchar,
        last_update -> Timestamp,
    }
}

diesel::table! {
    category (category_id) {
        category_id -> Int4,
        #[max_length = 25]
        name -> Varchar,
        last_update -> Timestamp,
    }
}

diesel::table! {
    city (city_id) {
        city_id -> Int4,
        #[max_length = 50]
        #[sql_name = "city"]
        city_col -> Varchar,
        country_id -> Int2,
        last_update -> Timestamp,
    }
}

diesel::table! {
    country (country_id) {
        country_id -> Int2,
        #[max_length = 50]
        #[sql_name = "country"]
        country_name -> Varchar,
        last_update -> Timestamp,
    }
}

diesel::table! {
    customer (customer_id) {
        customer_id -> Int4,
        store_id -> Int4,
        #[max_length = 45]
        first_name -> Varchar,
        #[max_length = 45]
        last_name -> Varchar,
        #[max_length = 50]
        email -> Nullable<Varchar>,
        address_id -> Int4,
        activebool -> Bool,
        create_date -> Date,
        last_update -> Nullable<Timestamp>,
        active -> Nullable<Int4>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::MpaaRating;
    use super::sql_types::Tsvector;

    film (film_id) {
        film_id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        description -> Nullable<Text>,
        release_year -> Nullable<Int4>,
        language_id -> Int4,
        rental_duration -> Int2,
        rental_rate -> Numeric,
        length -> Nullable<Int2>,
        replacement_cost -> Numeric,
        rating -> Nullable<MpaaRating>,
        last_update -> Timestamp,
        special_features -> Nullable<Array<Nullable<Text>>>,
        fulltext -> Tsvector,
    }
}

diesel::table! {
    film_actor (actor_id, film_id) {
        actor_id -> Int4,
        film_id -> Int4,
        last_update -> Timestamp,
    }
}

diesel::table! {
    film_category (film_id, category_id) {
        film_id -> Int4,
        category_id -> Int4,
        last_update -> Timestamp,
    }
}

diesel::table! {
    inventory (inventory_id) {
        inventory_id -> Int4,
        film_id -> Int4,
        store_id -> Int4,
        last_update -> Timestamp,
    }
}

diesel::table! {
    language (language_id) {
        language_id -> Int4,
        #[max_length = 20]
        name -> Bpchar,
        last_update -> Timestamp,
    }
}

diesel::table! {
    payment (payment_id) {
        payment_id -> Int4,
        customer_id -> Int4,
        staff_id -> Int4,
        rental_id -> Int4,
        amount -> Numeric,
        payment_date -> Timestamp,
    }
}

diesel::table! {
    rental (rental_id) {
        rental_id -> Int4,
        rental_date -> Timestamp,
        inventory_id -> Int4,
        customer_id -> Int4,
        return_date -> Nullable<Timestamp>,
        staff_id -> Int4,
        last_update -> Timestamp,
    }
}

diesel::table! {
    staff (staff_id) {
        staff_id -> Int4,
        #[max_length = 45]
        first_name -> Varchar,
        #[max_length = 45]
        last_name -> Varchar,
        address_id -> Int4,
        #[max_length = 50]
        email -> Nullable<Varchar>,
        store_id -> Int4,
        active -> Bool,
        #[max_length = 16]
        username -> Varchar,
        #[max_length = 40]
        password -> Nullable<Varchar>,
        last_update -> Timestamp,
        picture -> Nullable<Bytea>,
    }
}

diesel::table! {
    store (store_id) {
        store_id -> Int4,
        manager_staff_id -> Int4,
        address_id -> Int4,
        last_update -> Timestamp,
    }
}

diesel::joinable!(address -> city (city_id));
diesel::joinable!(city -> country (country_id));
diesel::joinable!(customer -> address (address_id));
diesel::joinable!(film -> language (language_id));
diesel::joinable!(film_actor -> actor (actor_id));
diesel::joinable!(film_actor -> film (film_id));
diesel::joinable!(film_category -> category (category_id));
diesel::joinable!(film_category -> film (film_id));
diesel::joinable!(inventory -> film (film_id));
diesel::joinable!(payment -> customer (customer_id));
diesel::joinable!(payment -> rental (rental_id));
diesel::joinable!(payment -> staff (staff_id));
diesel::joinable!(rental -> customer (customer_id));
diesel::joinable!(rental -> inventory (inventory_id));
diesel::joinable!(rental -> staff (staff_id));
diesel::joinable!(staff -> address (address_id));
diesel::joinable!(store -> address (address_id));
diesel::joinable!(store -> staff (manager_staff_id));

diesel::allow_tables_to_appear_in_same_query!(
    actor,
    address,
    category,
    city,
    country,
    customer,
    film,
    film_actor,
    film_category,
    inventory,
    language,
    payment,
    rental,
    staff,
    store,
);
