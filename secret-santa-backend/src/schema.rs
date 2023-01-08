// @generated automatically by Diesel CLI.

diesel::table! {
    members (id) {
        id -> Int4,
        user_id -> Int4,
        group_id -> Int4,
        urole -> Int4,
    }
}

diesel::table! {
    santas (id) {
        id -> Int4,
        group_id -> Int4,
        santa_id -> Int4,
        recipient_id -> Int4,
    }
}

diesel::table! {
    sgroups (id) {
        id -> Int4,
        gname -> Varchar,
        is_close -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::joinable!(members -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(members, santas, sgroups, users,);
