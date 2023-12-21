// use crate::repository::database::{Database,Repository};
// use crate::models::product::{Product,NewProduct}; 
// use crate::models::schema::product::dsl::*;
// use diesel::prelude::*; 
// pub struct ProductRepository {
//     pub db: Database,
// }

// impl Repository for ProductRepository {
//     fn new(db: Database) -> Self {
//         ProductRepository { db }
//     }
// }

// impl ProductRepository {
//      pub fn create_product(&self,product_i:NewProduct)->Result<Product,diesel::result::Error>{
//       diesel::insert_into(product).values(&product_i).get_result(&mut self.db.pool.get().unwrap()) 
//     } 
// }