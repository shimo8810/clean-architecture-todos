// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Varchar,
        body -> Text,
        state -> Bool,
    }
}
