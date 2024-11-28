// @generated automatically by Diesel CLI.

diesel::table! {
    authors (id) {
        id -> Text,
        firstname -> Text,
        lastname -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    auths (id) {
        id -> Text,
        username -> Text,
        password -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    book_favorites (id) {
        id -> Text,
        book_id -> Text,
        user_id -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    book_transaction_logs (id) {
        id -> Text,
        book_transaction_id -> Text,
        action -> Text,
        action_status -> Text,
        action_timestamp -> Timestamp,
        performed_by -> Text,
    }
}

diesel::table! {
    book_transactions (id) {
        id -> Text,
        book_id -> Text,
        user_id -> Text,
        transaction_type -> Text,
        transaction_date -> Timestamp,
        due_date -> Timestamp,
        return_date -> Nullable<Timestamp>,
        quantity -> Integer,
        created_at -> Timestamp,
    }
}

diesel::table! {
    books (id) {
        id -> Text,
        title -> Text,
        description -> Text,
        author_id -> Text,
        publication_date -> Timestamp,
        available_copies -> Integer,
        category_id -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    categories (id) {
        id -> Text,
        name -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        auth_id -> Text,
        firstname -> Text,
        lastname -> Text,
        email -> Text,
        created_at -> Timestamp,
    }
}

diesel::joinable!(book_favorites -> books (book_id));
diesel::joinable!(book_favorites -> users (user_id));
diesel::joinable!(book_transactions -> books (book_id));
diesel::joinable!(book_transactions -> users (user_id));
diesel::joinable!(books -> authors (author_id));
diesel::joinable!(books -> categories (category_id));
diesel::joinable!(users -> auths (auth_id));

diesel::allow_tables_to_appear_in_same_query!(
    authors,
    auths,
    book_favorites,
    book_transaction_logs,
    book_transactions,
    books,
    categories,
    users,
);
