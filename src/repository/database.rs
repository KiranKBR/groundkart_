use diesel::prelude::*; 
use diesel::r2d2::{self, ConnectionManager}; 
use dotenv::dotenv; 
use crate::models::product::{Product,NewProduct}; 
use crate::models::schema::product::dsl::*;

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct Database { 
    pub pool: DBPool, 
  }

  impl Database { 
    pub fn new() -> Self { 
      dotenv().ok(); 
      let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"); 
      let manager = ConnectionManager::<PgConnection>::new(database_url); 
      let result = r2d2::Pool::builder() 
                   .build(manager) 
                   .expect("Failed to create pool."); 
      Database { pool: result } 
    } 
  //1. add product
    pub fn create_product(&self,product_i:NewProduct)->Result<Product,diesel::result::Error>{ 
      diesel::insert_into(product).values(&product_i).get_result(&mut self.pool.get().unwrap()) 
    } 

    // 2. get all products
    pub fn get_products(&self) -> Vec<Product> { 
      product 
        .load::<Product>(&mut self.pool.get().unwrap()) 
        .expect("Failed to get events.") 
    } 
    //3. get product by id 
    pub fn get_product(&self, find_id:i32) -> Option<Product> { 
      product 
        .find(find_id) 
        .first::<Product>(&mut self.pool.get().unwrap()) 
        .ok() 
    }

    //4. update product 
    pub fn update_product(&self,_product:Product) -> Result<Product,diesel::result::Error>{
      diesel::update(product.filter(id.eq(_product.id))).set(&_product).get_result(&mut self.pool.get().unwrap()) 
    }

    //5. delete the product
    pub fn delete_product(&self,find_id:i32)->Result<usize,diesel::result::Error>{ 
      diesel::delete(product.filter(id.eq(find_id))).execute(&mut self.pool.get().unwrap()) 
    } 
  }

  pub trait Repository {
    fn new(db: Database) -> Self;
  }