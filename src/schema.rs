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
    color_history (id) {
        id -> Int4,
        history -> Array<Varchar>,
        change_timestamp -> Int8,
        register_timestamp -> Int8,
        userId -> Nullable<Int4>,
    }
}

table! {
    command (name) {
        name -> Varchar,
        counter -> Int4,
        permissions -> Int4,
        description -> Varchar,
        #[sql_name = "requiredParams"]
        required_params -> Array<Text>,
        #[sql_name = "optionalParams"]
        optional_params -> Array<Text>,
        cooldown -> Int4,
        deleted -> Bool,
        alias -> Nullable<Array<Varchar>>,
    }
}

table! {
    emotegame_stats (id) {
        id -> Int4,
        incorrect_guesses -> Int4,
        letters_guessed -> Int4,
        emotes_guessed -> Int4,
        userId -> Nullable<Int4>,
    }
}

table! {
    error (id) {
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
    notification (id) {
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
        #[sql_name = "suggestion"]
        suggestion_info -> Varchar,
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
    }
}

table! {
    wordle_words (word) {
        word -> Varchar,
        is_answer -> Bool,
    }
}

table! {
    twitch_tokens (id) {
        id -> Int4,
        token -> Varchar,
        #[sql_name = "userId"]
        user_id -> Nullable<Int4>,
    }
}

joinable!(color_history -> user (userId));
joinable!(emotegame_stats -> user (userId));
joinable!(notification -> user (user_id));
joinable!(suggestion -> user (userId));
joinable!(twitch_tokens -> user (user_id));

allow_tables_to_appear_in_same_query!(
    ban,
    channel,
    color_history,
    command,
    emotegame_stats,
    error,
    migrations,
    notification,
    notification_channel,
    suggestion,
    timeout,
    twitch_tokens,
    user,
    wordle_words,
);
