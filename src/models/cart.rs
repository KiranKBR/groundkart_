// cart.rs
use diesel::Queryable;

#[derive(Debug, Queryable)]
pub struct Cart {
    pub id: i32,
}
