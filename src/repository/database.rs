use diesel::prelude::*; 
use diesel::r2d2::{self, ConnectionManager}; 
use dotenv::dotenv; 
use crate::models::customer::{self, Customer, NewCustomer};
use crate::models::cart::Cart;
use crate::models::cartitem::{CartItem,NewCartItem};
use crate::models::product::{Product,NewProduct}; 
use crate::models::schema::product::dsl as product_schema;
use crate::models::schema::customer::dsl as customer_schema;
use crate::models::schema::cart::dsl as cart_schema;
use crate::models::schema::cartitem::dsl::{self as cartitem_schema, quantity};
use diesel::associations::HasTable;


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
      diesel::insert_into(product_schema::product).values(&product_i).get_result(&mut self.pool.get().unwrap()) 
    } 

    // 2. get all products
    pub fn get_products(&self) -> Vec<Product> { 
      product_schema::product 
        .load::<Product>(&mut self.pool.get().unwrap()) 
        .expect("Failed to get products.") 
    } 
    //3. get product by id 
    pub fn get_product(&self, find_id:i32) -> Option<Product> { 
      product_schema::product 
        .find(find_id) 
        .first::<Product>(&mut self.pool.get().unwrap()) 
        .ok() 
    }

    //4. update product 
    pub fn update_product(&self,_product:Product) -> Result<Product,diesel::result::Error>{
      diesel::update(product_schema::product.filter(product_schema::id.eq(_product.id))).set(&_product).get_result(&mut self.pool.get().unwrap()) 
    }

    //5. delete the product
    pub fn delete_product(&self,find_id:i32)->Result<usize,diesel::result::Error>{ 
      diesel::delete(product_schema::product.filter(product_schema::id.eq(find_id))).execute(&mut self.pool.get().unwrap()) 
    } 
        // 6. get cart
    pub fn get_cart(&self, user_id: i32) -> Option<Vec<CartItem>> {
      let cart_id = customer_schema::customer
      .find(user_id)
      .select(customer_schema::cart_id)
      .first::<Option<i32>>(&mut self.pool.get().unwrap())
      .expect("Failed to get user's cart ID")?;

  cartitem_schema::cartitem
      .filter(cartitem_schema::cart_id.eq(cart_id))
      .load::<CartItem>(&mut self.pool.get().unwrap())
      .ok()
  }    

    //7. get user 
    pub fn get_user(&self,user_id:i32) -> Option<Customer>
    {
      customer_schema::customer
        .find(user_id)
        .first::<Customer>(&mut self.pool.get().unwrap())
        .ok()
    } 

    //8.get users
    pub fn get_users(&self)-> Vec<Customer> {
      customer_schema::customer 
      .load::<Customer>(&mut self.pool.get().unwrap()) 
      .expect("Failed to get users.") 
    }

    //9.add user
    pub fn add_user(&self,user:NewCustomer)->Result<Customer,diesel::result::Error>{ 
      let cart:Cart = diesel::insert_into(cart_schema::cart)
      .default_values()
      .get_result(&mut self.pool.get().unwrap()).expect("error");
      // diesel::insert_into(customer_schema::customer).values(&user).get_result(&mut self.pool.get().unwrap()) 
      let mut customer:Customer = diesel::insert_into(customer_schema::customer)
        .values(&user)
        // .set(customer_schema::cart_id.eq(cart.id+1)) // Use `last_insert_rowid()` to get the generated ID
        .get_result(&mut self.pool.get().unwrap())?;
        customer.cart_id= Some(cart.id);
        diesel::update(customer_schema::customer.filter(customer_schema::id.eq(customer.id))).set(&customer).get_result(&mut self.pool.get().unwrap())
    } 

    //10.add to cart
    pub fn add_to_cart(&self,user_id:i32,productid:i32) -> Result<CartItem,diesel::result::Error> {
      let user:Customer = self.get_user(user_id).unwrap();
      let cart_item:NewCartItem = NewCartItem {
          quantity: Some(1),
          product_id: Some(productid),
          cart_id:user.cart_id
      };
      diesel::insert_into(cartitem_schema::cartitem).values(&cart_item).get_result(&mut self.pool.get().unwrap()) 

    }

    //11.remove from cart
    pub fn delete_cartitem(&self,cartitemid:i32) -> Result<usize,diesel::result::Error> {
      diesel::delete(cartitem_schema::cartitem.filter(cartitem_schema::id.eq(cartitemid))).execute(&mut self.pool.get().unwrap()) 
    }

    //12.update cart item 
    pub fn update_cart_item(&self,item:CartItem) -> Result<CartItem,diesel::result::Error> {
      diesel::update(cartitem_schema::cartitem.filter(cartitem_schema::id.eq(item.id))).set(&item).get_result(&mut self.pool.get().unwrap()) 
    }
  }

  pub trait Repository {
    fn new(db: Database) -> Self;
  }