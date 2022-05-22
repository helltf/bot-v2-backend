table! {
    ban (id) {
        id -> Int4,
        user -> Varchar,
        channel -> Varchar,
        at -> Int8,
    }
}

table! {
    channel (id) {
        id -> Int4,
        channel -> Varchar,
        allowed -> Bool,
        allowed_live -> Bool,
        joined -> Bool,
        times_connected -> Int4,
        connect_timestamp -> Int8,
    }
}

table! {
    color_history (id) {
        id -> Int4,
        history -> Array<Varchar>,
        change_timestamp -> Int8,
        register_timestamp -> Int8,
    }
}

table! {
    command (name) {
        name -> Varchar,
        counter -> Int4,
        permissions -> Int4,
        description -> Varchar,
        requiredParams -> Array<Varchar>,
        optionalParams -> Array<Varchar>,
        cooldown -> Int4,
    }
}

table! {
    notification (id) {
        id -> Int4,
        streamer -> Varchar,
        channel -> Varchar,
        live -> Bool,
        offline -> Bool,
        title -> Bool,
        game -> Bool,
        userId -> Nullable<Int4>,
    }
}

table! {
    notification_channel (id) {
        id -> Int4,
        name -> Varchar,
        status -> Bool,
        setting -> Bool,
    }
}

table! {
    suggestion (id) {
        id -> Int4,
        suggestion -> Varchar,
        date -> Int8,
        userId -> Nullable<Int4>,
    }
}

table! {
    timeout (id) {
        id -> Int4,
        user -> Varchar,
        channel -> Varchar,
        at -> Int8,
        duration -> Int8,
    }
}

table! {
    user (id) {
        id -> Int4,
        name -> Varchar,
        color -> Nullable<Varchar>,
        permission -> Int4,
        registered_at -> Int8,
        display_name -> Varchar,
        colorsId -> Nullable<Int4>,
    }
}

table! {
    watchchannel (channel) {
        channel -> Varchar,
        joined_date -> Int8,
    }
}

table! {
    wordle_words (word) {
        word -> Varchar,
        is_answer -> Bool,
    }
}

joinable!(notification -> user (userId));
joinable!(suggestion -> user (userId));
joinable!(user -> color_history (colorsId));

allow_tables_to_appear_in_same_query!(
    ban,
    channel,
    color_history,
    command,
    notification,
    notification_channel,
    suggestion,
    timeout,
    user,
    watchchannel,
    wordle_words,
);
