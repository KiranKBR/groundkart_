// orderitem.rs
use diesel::{Queryable, Insertable,AsChangeset}; 
use diesel::Associations;
use diesel::Identifiable;
use serde::{Deserialize, Serialize}; 


use super::product::Product;
use super::salesorder::SalesOrder;

#[derive(Debug, Queryable, Associations, Identifiable, Deserialize, Serialize,Insertable)]
#[diesel(belongs_to(Product))]
#[belongs_to(SalesOrder, foreign_key = "salesorder_id")]
#[diesel(table_name=crate::models::schema::orderitem)] 
pub struct OrderItem {
    pub id: i32,
    pub quantity: Option<i32>,
    pub price: Option<f64>,
    pub product_id: Option<i32>,
    pub salesorder_id: Option<i32>,
}
#[derive(Deserialize, Serialize,Debug,Clone,Insertable)] 
#[diesel(table_name=crate::models::schema::orderitem)] 
pub struct NewOrderItem {
    pub quantity: Option<i32>,
    pub price: Option<f64>,
    pub product_id: Option<i32>,
    pub salesorder_id: Option<i32>,
}