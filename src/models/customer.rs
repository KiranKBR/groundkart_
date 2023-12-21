// customer.rs
use serde::{Deserialize, Serialize}; 
use diesel::{Queryable, Insertable,AsChangeset}; 
use diesel::Identifiable;





#[derive(Queryable, Serialize, Deserialize,Debug,Clone,AsChangeset,Insertable, Identifiable)] 
#[diesel(belongs_to(Cart))]
#[diesel(table_name=crate::models::schema::customer)] 
pub struct Customer {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub password: String,
    pub phone: String,
    pub cart_id: Option<i32>,
}
