table! {
    appointments (id) {
        id -> Integer,
        date -> Text,
        start_time -> Text,
        end_time -> Text,
        books_id -> Integer,
        proposer_id -> Integer,
        room_id -> Nullable<Integer>,
        state -> Text,
    }
}

table! {
    books (id) {
        id -> Integer,
        url -> Text,
        course_id -> Integer,
        customer_id -> Integer,
    }
}

table! {
    conducts (provider_id, course_id) {
        provider_id -> Integer,
        course_id -> Integer,
    }
}

table! {
    courses (id) {
        id -> Integer,
        name -> Text,
        default_duration -> Nullable<Integer>,
        default_room_id -> Nullable<Integer>,
        course_type -> Nullable<Text>,
    }
}

table! {
    customers (person_id) {
        person_id -> Integer,
        organisation -> Text,
        class -> Nullable<Text>,
    }
}

table! {
    persons (id) {
        id -> Integer,
        firstname -> Text,
        lastname -> Text,
        email -> Text,
        phone -> Nullable<Text>,
    }
}

table! {
    providers (person_id) {
        person_id -> Integer,
        password_hash -> Nullable<Text>,
        is_admin -> Integer,
    }
}

table! {
    rooms (id) {
        id -> Integer,
        name -> Text,
    }
}

joinable!(appointments -> books (books_id));
joinable!(appointments -> persons (proposer_id));
joinable!(appointments -> rooms (room_id));
joinable!(books -> courses (course_id));
joinable!(books -> customers (customer_id));
joinable!(courses -> rooms (default_room_id));
joinable!(customers -> persons (person_id));
joinable!(providers -> persons (person_id));

allow_tables_to_appear_in_same_query!(
    appointments,
    books,
    conducts,
    courses,
    customers,
    persons,
    providers,
    rooms,
);
