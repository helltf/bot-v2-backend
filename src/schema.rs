table! {
    bans (id) {
        id -> Int4,
        user -> Varchar,
        channel -> Varchar,
        at -> Int8,
    }
}

table! {
    channels (id) {
        id -> Int4,
        #[sql_name = "channel"]
        channel_name -> Varchar,
        allowed -> Bool,
        allowed_live -> Bool,
        joined -> Bool,
        times_connected -> Int4,
        connect_timestamp -> Int8,
    }
}

table! {
    color_histories (id) {
        id -> Int4,
        history -> Array<Varchar>,
        change_timestamp -> Int8,
        register_timestamp -> Int8,
        #[sql_name = "userId"]
        user_id -> Nullable<Int4>,
    }
}

table! {
    commands (name) {
        name -> Varchar,
        counter -> Int4,
        permissions -> Int4,
        description -> Varchar,
        cooldown -> Int4,
        deleted -> Bool,
        alias -> Nullable<Array<Varchar>>,
        #[sql_name = "requiredParams"]
        required_params -> Array<Text>,
        #[sql_name = "optionalParams"]
        optional_params -> Array<Text>,
    }
}

table! {
    emotegame_stats (id) {
        id -> Int4,
        incorrect_guesses -> Int4,
        letters_guessed -> Int4,
        emotes_guessed -> Int4,
        #[sql_name = "userId"]
        user_id -> Nullable<Int4>,
    }
}

table! {
    errors (id) {
        id -> Int4,
        message -> Nullable<Varchar>,
        stack_trace -> Nullable<Varchar>,
        timestamp -> Int8,
    }
}

table! {
    migrations (id) {
        id -> Int4,
        timestamp -> Int8,
        name -> Varchar,
    }
}

table! {
    notification_channels (id) {
        id -> Int4,
        name -> Varchar,
        status -> Bool,
        setting -> Bool,
    }
}

table! {
    notifications (id) {
        id -> Int4,
        streamer -> Varchar,
        channel -> Varchar,
        live -> Bool,
        offline -> Bool,
        title -> Bool,
        game -> Bool,
        #[sql_name = "userId"]
        user_id -> Nullable<Int4>,
    }
}

table! {
    suggestions (id) {
        id -> Int4,
        suggestion -> Varchar,
        date -> Int8,
        #[sql_name = "userId"]
        user_id -> Nullable<Int4>,
        status -> Varchar,
        reason -> Nullable<Varchar>,
    }
}

table! {
    timeouts (id) {
        id -> Int4,
        user -> Varchar,
        channel -> Varchar,
        at -> Int8,
        duration -> Int8,
    }
}

table! {
    twitch_tokens (id) {
        id -> Int4,
        token -> Bytea,
        nonce -> Bytea,
        refresh_token -> Varchar,
        #[sql_name = "userId"]
        user_id -> Nullable<Int4>,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        color -> Nullable<Varchar>,
        permission -> Int4,
        registered_at -> Int8,
        display_name -> Varchar,
    }
}

table! {
    wordle_words (word) {
        word -> Varchar,
        is_answer -> Bool,
    }
}

joinable!(color_histories -> users (user_id));
joinable!(emotegame_stats -> users (user_id));
joinable!(notifications -> users (user_id));
joinable!(suggestions -> users (user_id));
joinable!(twitch_tokens -> users (user_id));

allow_tables_to_appear_in_same_query!(
    bans,
    channels,
    color_histories,
    commands,
    emotegame_stats,
    errors,
    migrations,
    notification_channels,
    notifications,
    suggestions,
    timeouts,
    twitch_tokens,
    users,
    wordle_words,
);
