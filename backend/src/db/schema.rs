// @generated automatically by Diesel CLI.

diesel::table! {
    appointments (id) {
        id -> Integer,
        start_time -> Text,
        end_time -> Text,
        books_id -> Integer,
        proposer_id -> Integer,
        room_id -> Integer,
        state -> Text,
    }
}

diesel::table! {
    books (id) {
        id -> Integer,
        url -> Text,
        course_id -> Integer,
        customer_id -> Integer,
    }
}

diesel::table! {
    conducts (provider_id, course_id) {
        provider_id -> Integer,
        course_id -> Integer,
    }
}

diesel::table! {
    courses (id) {
        id -> Integer,
        name -> Text,
        default_duration -> Nullable<Integer>,
        default_room_id -> Integer,
        course_type -> Nullable<Text>,
    }
}

diesel::table! {
    customers (person_id) {
        person_id -> Integer,
        organisation -> Text,
        class -> Nullable<Text>,
    }
}

diesel::table! {
    persons (id) {
        id -> Integer,
        firstname -> Text,
        lastname -> Text,
        email -> Text,
        phone -> Nullable<Text>,
    }
}

diesel::table! {
    providers (person_id) {
        person_id -> Integer,
        password_hash -> Nullable<Text>,
        is_admin -> Integer,
    }
}

diesel::table! {
    rooms (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::joinable!(appointments -> books (books_id));
diesel::joinable!(appointments -> persons (proposer_id));
diesel::joinable!(appointments -> rooms (room_id));
diesel::joinable!(books -> courses (course_id));
diesel::joinable!(books -> customers (customer_id));
diesel::joinable!(courses -> rooms (default_room_id));
diesel::joinable!(customers -> persons (person_id));
diesel::joinable!(providers -> persons (person_id));

diesel::allow_tables_to_appear_in_same_query!(
    appointments,
    books,
    conducts,
    courses,
    customers,
    persons,
    providers,
    rooms,
);
