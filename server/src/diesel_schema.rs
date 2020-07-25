table! {
    to_dos (id) {
        id -> Text,
        label -> Text,
    }
}

table! {
    todo (id) {
        id -> Text,
        label -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    to_dos,
    todo,
);
