// @generated automatically by Diesel CLI.

diesel::table! {
    artists (id) {
        id -> Integer,
        name -> Text,
        url_bandcamp -> Nullable<Text>,
        url_metallum -> Text,
    }
}

diesel::table! {
    feeds (id) {
        id -> Integer,
        date -> Integer,
        feed -> Text,
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
        url_youtube -> Text,
        url_metallum -> Text,
    }
}

diesel::joinable!(releases -> artists (artist_id));

diesel::allow_tables_to_appear_in_same_query!(artists, feeds, releases,);
