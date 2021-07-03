table! {
    pub users (id) {
        id -> Unsigned<Bigint>,
        user_name -> Varchar,
        password -> Varchar,
        token -> Nullable<Varchar>,
    }
}
