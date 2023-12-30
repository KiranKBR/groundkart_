// salesorder.rs
use serde::{Deserialize, Serialize}; 
use diesel::{Queryable, Insertable,AsChangeset}; 
use diesel::Identifiable;
// use diesel::BelongingToDsl;


#[derive(Debug, Queryable, Identifiable,AsChangeset,Serialize,Deserialize)]
#[diesel(belongs_to(Customer))]
#[diesel(table_name=crate::models::schema::salesorder)] 
pub struct SalesOrder {
    pub id: i32,
    pub price: Option<f64>,
    pub customer_id: Option<i32>,
}
#[derive(Deserialize, Serialize,Debug,Clone,Insertable)] 
#[diesel(table_name=crate::models::schema::salesorder)] 
pub struct NewSalesOrder {
    pub price: Option<f64>,
    pub customer_id: Option<i32>,
}
