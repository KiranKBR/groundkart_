use diesel::prelude::*; 
use diesel::r2d2::{self, ConnectionManager}; 
use dotenv::dotenv; 
use crate::models::customer::{self, Customer, NewCustomer};
use crate::models::cart::Cart;
use crate::models::cartitem::{CartItem,NewCartItem};
use crate::models::orderitem::{OrderItem, NewOrderItem};
use crate::models::product::{Product,NewProduct}; 
use crate::models::schema::product::dsl::{self as product_schema, unit_stock};
use crate::models::schema::customer::dsl as customer_schema;
use crate::models::schema::cart::dsl as cart_schema;
use crate::models::schema::orderitem::dsl as orderitem_schema;
use crate::models::schema::salesorder::dsl as order_schema;
use crate::models::schema::cartitem::dsl::{self as cartitem_schema, quantity};
use diesel::associations::HasTable;
use crate::models::salesorder::{SalesOrder,NewSalesOrder};



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
    //6. get cart
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

    //13. direct buy
    pub fn direct_buy(&self,user_id:i32,productid:i32) -> Result<OrderItem,diesel::result::Error> {
      let mut product = self.get_product(productid).unwrap();
      let order = NewSalesOrder {
          customer_id:Some(user_id),
          price:Some(product.price.unwrap())
      }; 
      let order:Result<SalesOrder,diesel::result::Error> = diesel::insert_into(order_schema::salesorder).values(&order).get_result(&mut self.pool.get().unwrap());
      let item = NewOrderItem {
          quantity:Some(1),
          price:Some(product.price.unwrap()),
          product_id:Some(productid),
          salesorder_id:Some(order.unwrap().id)
      };
      product.unit_stock=Some(product.unit_stock.unwrap()-1);
      let _ = self.update_product(product);
      diesel::insert_into(orderitem_schema::orderitem).values(&
        item).get_result(&mut self.pool.get().unwrap()) 
    }

    //14.buy cart item
    pub fn buy_cart_item(&self,item:CartItem) -> Result<OrderItem,diesel::result::Error> {
      let mut product = self.get_product(item.product_id.unwrap()).unwrap();
      let order = NewSalesOrder {
        customer_id:Some(1),
        price:Some(item.quantity.unwrap() as f64 *product.price.unwrap()),
      };
      product.unit_stock = Some(product.unit_stock.unwrap()-item.quantity.unwrap());
      let i_id=item.id;
      let order:SalesOrder = diesel::insert_into(order_schema::salesorder).values(&order).get_result(&mut self.pool.get().unwrap())?;
      let item = NewOrderItem {
        quantity:Some(item.quantity.unwrap()),
        price:Some(order.price.unwrap()),
        product_id:Some(product.id),
        salesorder_id:Some(order.id)
    };
    let _ = self.update_product(product);
    let _= self.delete_cartitem(i_id);
    diesel::insert_into(orderitem_schema::orderitem).values(&
      item).get_result(&mut self.pool.get().unwrap()) 
    }

    //15.checkout 
    pub fn checkout_cart(&self,user_id: i32) -> Result<SalesOrder,diesel::result::Error> {
      let items = self.get_cart(user_id).unwrap();
      let order = NewSalesOrder {
        customer_id:Some(user_id),
        price:None,
      };
      let mut order:SalesOrder = diesel::insert_into(order_schema::salesorder).values(&order).get_result(&mut self.pool.get().unwrap())?;
      let mut price:f64 = 0.0;
      for item in &items {
        let i_id=item.id;
        let mut product = self.get_product(item.product_id.unwrap()).unwrap();
        let tprice = item.quantity.unwrap() as f64 *product.price.unwrap(); 
        product.unit_stock = Some(product.unit_stock.unwrap()-item.quantity.unwrap());
        price = price+tprice;
        let item :NewOrderItem= NewOrderItem {
          quantity:Some(item.quantity.unwrap()),
          price:Some(tprice),
          product_id:Some(product.id),
          salesorder_id:Some(order.id)
        };
        let _ = self.update_product(product);
        let _= self.delete_cartitem(i_id);
        let _ = diesel::insert_into(orderitem_schema::orderitem).values(&
           item).execute(&mut self.pool.get().unwrap());
      }
      order.price=Some(price);
      diesel::update(order_schema::salesorder.filter(order_schema::id.eq(order.id))).set(&order).get_result(&mut self.pool.get().unwrap())
    }

    //16.get order list of user
    pub fn get_orders(&self,user_id:i32) -> Option<Vec<SalesOrder>> {
      order_schema::salesorder
        .filter(order_schema::customer_id.eq(user_id))
        .load::<SalesOrder>(&mut self.pool.get().unwrap())
        .ok()
    }

    //17.get order items by id 
    pub fn get_order_by_id(&self,order_id:i32) -> Option<Vec<OrderItem>> {
      orderitem_schema::orderitem
        .filter(orderitem_schema::salesorder_id.eq(order_id))
        .load::<OrderItem>(&mut self.pool.get().unwrap())
        .ok()
    }
  }

  pub trait Repository {
    fn new(db: Database) -> Self;
  }