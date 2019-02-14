table! {
    posts (id) {
        id -> Integer,
        timestamp -> Timestamp,
        author -> Text,
        body -> Text,
        like -> Integer,
        unlike -> Integer,
    }
}
