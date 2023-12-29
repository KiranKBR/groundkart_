use serde::{Deserialize, Serialize}; 
use diesel::{Queryable, Insertable,AsChangeset}; // cartitem.rs
use diesel::Associations;
use diesel::Identifiable;
// use diesel::BelongingToDsl;

use super::cart::Cart;
use super::product::Product;

#[derive(Debug, Queryable, Associations, Identifiable,Deserialize,Serialize,AsChangeset)]
#[diesel(belongs_to(Cart))]
#[diesel(belongs_to(Product))]
#[diesel(table_name=crate::models::schema::cartitem)] 

// #[derive(Deserialize, Serialize,Debug,Clone,Insertable)] 
pub struct CartItem {
    pub id: i32,
    pub quantity: Option<i32>,
    pub product_id: Option<i32>,
    pub cart_id: Option<i32>,
}

#[derive(Deserialize, Serialize,Debug,Clone,Insertable)] 
#[diesel(table_name=crate::models::schema::cartitem)] 
pub struct NewCartItem {
    pub quantity: Option<i32>,
    pub product_id: Option<i32>,
    pub cart_id: Option<i32>,
}
