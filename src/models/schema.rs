// @generated automatically by Diesel CLI.

diesel::table! {
    cart (id) {
        id -> Int4,
    }
}

diesel::table! {
    cartitem (id) {
        id -> Int4,
        quantity -> Nullable<Int4>,
        product_id -> Nullable<Int4>,
        cart_id -> Nullable<Int4>,
    }
}

diesel::table! {
    customer (id) {
        id -> Int4,
        email -> Text,
        name -> Text,
        password -> Text,
        phone -> Text,
        cart_id -> Nullable<Int4>,
    }
}

diesel::table! {
    orderitem (id) {
        id -> Int4,
        quantity -> Nullable<Int4>,
        price -> Nullable<Float8>,
        product_id -> Nullable<Int4>,
        salesorder_id -> Nullable<Int4>,
    }
}

diesel::table! {
    product (id) {
        id -> Int4,
        category -> Text,
        name -> Text,
        unit_stock -> Nullable<Int4>,
    }
}

diesel::table! {
    salesorder (id) {
        id -> Int4,
        price -> Nullable<Float8>,
        customer_id -> Nullable<Int4>,
    }
}

diesel::joinable!(cartitem -> cart (cart_id));
diesel::joinable!(cartitem -> product (product_id));
diesel::joinable!(customer -> cart (cart_id));
diesel::joinable!(orderitem -> product (product_id));
diesel::joinable!(orderitem -> salesorder (salesorder_id));
diesel::joinable!(salesorder -> customer (customer_id));

diesel::allow_tables_to_appear_in_same_query!(
    cart,
    cartitem,
    customer,
    orderitem,
    product,
    salesorder,
);
