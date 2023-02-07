// @generated automatically by Diesel CLI.

diesel::table! {
    config (name) {
        name -> Varchar,
        value -> Text,
    }
}
