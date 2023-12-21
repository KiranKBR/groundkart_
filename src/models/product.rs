// product.rs
use serde::{Deserialize, Serialize}; 
use diesel::{Queryable, Insertable,AsChangeset}; 

#[derive(Queryable, Serialize, Deserialize,Debug,Clone,AsChangeset,Insertable)] 
#[diesel(table_name=crate::models::schema::product)] 
pub struct Product {
    pub id: i32,
    pub category: String,
    pub name: String,
    pub unit_stock: Option<i32>,
}

#[derive(Deserialize, Serialize,Debug,Clone,Insertable)] 
#[diesel(table_name=crate::models::schema::product)] 
pub struct NewProduct {
    pub category: String,
    pub name: String,
    pub unit_stock: Option<i32>,
}