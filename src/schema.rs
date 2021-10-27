table! {
    fixtures (id) {
        id -> Integer,
        name -> Text,
        link -> Text,
        team1 -> Text,
        team2 -> Text,
        start_time -> Timestamp,
        rating -> Integer,
        meta -> Text,
        analytics -> Text,
        top_tier -> Bool,
        hash -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    news (id) {
        id -> Integer,
        title -> Text,
        link -> Text,
        pub_date -> Timestamp,
        image -> Nullable<Text>,
        image_text -> Nullable<Text>,
        content -> Nullable<Text>,
        description -> Nullable<Text>,
        author -> Nullable<Text>,
        hash -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(fixtures, news,);
