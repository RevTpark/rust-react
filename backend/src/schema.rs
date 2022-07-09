table! {
    products (id) {
        id -> Int4,
        name -> Varchar,
        price -> Int4,
        description -> Varchar,
        created_by -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        role -> Varchar,
        email -> Varchar,
        password -> Varchar,
        api_key -> Varchar,
    }
}

joinable!(products -> users (created_by));

allow_tables_to_appear_in_same_query!(
    products,
    users,
);
