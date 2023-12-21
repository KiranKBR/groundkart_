// cartitem.rs
use diesel::Queryable;
use diesel::Associations;
use diesel::Identifiable;
use diesel::BelongingToDsl;

use super::cart::Cart;
use super::product::Product;

#[derive(Debug, Queryable, Associations, Identifiable)]
#[diesel(belongs_to(Cart))]
#[diesel(belongs_to(Product))]
#[diesel(table_name=crate::models::schema::cartitem)] 

pub struct CartItem {
    pub id: i32,
    pub quantity: Option<i32>,
    pub product_id: Option<i32>,
    pub cart_id: Option<i32>,
}
