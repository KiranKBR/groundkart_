// salesorder.rs
use diesel::Queryable;
use diesel::Associations;
use diesel::Identifiable;
// use diesel::BelongingToDsl;

use super::customer::Customer;

#[derive(Debug, Queryable, Associations, Identifiable)]
#[belongs_to(Customer, foreign_key = "customer_id")]
#[diesel(table_name=crate::models::schema::salesorder)] 
pub struct SalesOrder {
    pub id: i32,
    pub price: Option<f64>,
    pub customer_id: Option<i32>,
}
