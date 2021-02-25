table! {
    invitations (id) {
        id -> Uuid,
        email -> Varchar,
        expires_at -> Timestamp,
    }
}

table! {
    users (email) {
        email -> Varchar,
        hash -> Varchar,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(invitations, users,);
