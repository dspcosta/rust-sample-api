// @generated automatically by Diesel CLI.

diesel::table! {
    customer (customer_id) {
        customer_id -> Int4,
        store_id -> Int2,
        #[max_length = 45]
        first_name -> Varchar,
        #[max_length = 45]
        last_name -> Varchar,
        #[max_length = 50]
        email -> Nullable<Varchar>,
        address_id -> Int2,
        activebool -> Bool,
        create_date -> Date,
        last_update -> Nullable<Timestamp>,
        active -> Nullable<Int4>,
    }
}
