// @generated automatically by Diesel CLI.

diesel::table! {
    Chat (id) {
        id -> Uuid,
        createdAt -> Timestamp,
        userId -> Uuid,
        title -> Text,
        visibility -> Varchar,
        lastContext -> Nullable<Jsonb>,
    }
}

diesel::table! {
    Document (id, createdAt) {
        id -> Uuid,
        createdAt -> Timestamp,
        title -> Text,
        content -> Nullable<Text>,
        userId -> Uuid,
        text -> Varchar,
    }
}

diesel::table! {
    Message (id) {
        id -> Uuid,
        chatId -> Uuid,
        role -> Varchar,
        content -> Json,
        createdAt -> Timestamp,
    }
}

diesel::table! {
    Message_v2 (id) {
        id -> Uuid,
        chatId -> Uuid,
        role -> Varchar,
        parts -> Json,
        attachments -> Json,
        createdAt -> Timestamp,
    }
}

diesel::table! {
    Stream (id) {
        id -> Uuid,
        chatId -> Uuid,
        createdAt -> Timestamp,
    }
}

diesel::table! {
    Suggestion (id) {
        id -> Uuid,
        documentId -> Uuid,
        documentCreatedAt -> Timestamp,
        originalText -> Text,
        suggestedText -> Text,
        description -> Nullable<Text>,
        isResolved -> Bool,
        userId -> Uuid,
        createdAt -> Timestamp,
    }
}

diesel::table! {
    User (id) {
        id -> Uuid,
        #[max_length = 64]
        email -> Varchar,
        #[max_length = 64]
        password -> Nullable<Varchar>,
    }
}

diesel::table! {
    Vote (chatId, messageId) {
        chatId -> Uuid,
        messageId -> Uuid,
        isUpvoted -> Bool,
    }
}

diesel::table! {
    Vote_v2 (chatId, messageId) {
        chatId -> Uuid,
        messageId -> Uuid,
        isUpvoted -> Bool,
    }
}

diesel::table! {
    books (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 255]
        author -> Nullable<Varchar>,
        publication_year -> Nullable<Int4>,
        in_stock -> Nullable<Bool>,
    }
}

diesel::table! {
    customers (member_id) {
        member_id -> Int4,
        name -> Varchar,
        password -> Varchar,
        email -> Varchar,
    }
}

diesel::table! {
    playing_with_neon (id) {
        id -> Int4,
        name -> Text,
        value -> Nullable<Float4>,
    }
}

diesel::joinable!(Chat -> User (userId));
diesel::joinable!(Document -> User (userId));
diesel::joinable!(Message -> Chat (chatId));
diesel::joinable!(Message_v2 -> Chat (chatId));
diesel::joinable!(Stream -> Chat (chatId));
diesel::joinable!(Suggestion -> User (userId));
diesel::joinable!(Vote -> Chat (chatId));
diesel::joinable!(Vote -> Message (messageId));
diesel::joinable!(Vote_v2 -> Chat (chatId));
diesel::joinable!(Vote_v2 -> Message_v2 (messageId));

diesel::allow_tables_to_appear_in_same_query!(
    Chat,
    Document,
    Message,
    Message_v2,
    Stream,
    Suggestion,
    User,
    Vote,
    Vote_v2,
    books,
    customers,
    playing_with_neon,
);
