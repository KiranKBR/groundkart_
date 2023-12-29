// cart.rs

use serde::{Deserialize, Serialize}; 
use diesel::{Queryable, Insertable,AsChangeset}; 
use diesel::Identifiable;

#[derive(Deserialize, Serialize,Debug,Clone,Insertable,Queryable,Identifiable)] 
#[diesel(table_name=crate::models::schema::cart)] 
pub struct Cart {
    pub id: i32,
}

