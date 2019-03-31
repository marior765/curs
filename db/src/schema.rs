table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}


table! {
    heroes {
        id -> Nullable<Integer>,
        name -> Varchar,
        identity -> Varchar,
        hometown -> Varchar,
        age -> Integer,
    }
}
