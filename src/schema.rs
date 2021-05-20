table! {
    trees (id) {
        id -> Varchar,
        name -> Varchar,
        genus -> Varchar,
        lat -> Varchar,
        lng -> Varchar,
    }
}

table! {
    users (id) {
        id -> Varchar,
        name -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    trees,
    users,
);
