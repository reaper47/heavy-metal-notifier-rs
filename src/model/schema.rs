// @generated automatically by Diesel CLI.

diesel::table! {
    artists (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    links (id) {
        id -> Integer,
        artist_id -> Integer,
        url_youtube -> Text,
        url_bandcamp -> Nullable<Text>,
    }
}

diesel::table! {
    releases (id) {
        id -> Integer,
        year -> Integer,
        month -> Integer,
        day -> Integer,
        artist_id -> Integer,
        album -> Text,
    }
}

diesel::joinable!(links -> artists (artist_id));
diesel::joinable!(releases -> artists (artist_id));

diesel::allow_tables_to_appear_in_same_query!(artists, links, releases,);
