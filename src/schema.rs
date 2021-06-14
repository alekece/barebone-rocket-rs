table! {
    users (username) {
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        is_admin -> Bool,
        token -> Nullable<Varchar>,
    }
}
