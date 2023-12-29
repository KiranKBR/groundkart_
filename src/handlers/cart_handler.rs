

use actix_web::{web,get,post,HttpResponse,delete,put};
// use diesel::result::Error::NotFound; 
// use crate::models::product::{Product,NewProduct, self};
use crate::{repository::database::Database, models::cartitem::CartItem};
// use env_logger::Env;

#[get("/getcart/{userid}")]
async fn get_cart(db:web::Data<Database>,path:web::Path<i32>) -> HttpResponse {
    let cart = db.get_cart(path.into_inner());
    HttpResponse::Ok().json(cart)
}

#[post("/addtocart/{userid}/{productid}")]
async fn add_to_cart(db:web::Data<Database>,path:web::Path<(i32,i32)>) -> HttpResponse {
  let cart_item = db.add_to_cart(path.0,path.1);
  match cart_item {
    Ok(cart_item) =>HttpResponse::Ok().json(cart_item),
    Err(e) => HttpResponse::NotFound().body("not found")
  }
}

#[delete("removefromcart/{cartitemid}")]
async fn remove_from_cart(db:web::Data<Database>,path:web::Path<i32>) -> HttpResponse {
  let cartitem = db.delete_cartitem(path.into_inner()); 
  match cartitem { 
    Ok(cartitem)=>HttpResponse::Ok().json(cartitem), 
    Err(_)=>HttpResponse::InternalServerError().body("Internal Server Error") 
  } 
}

#[put("/updatecart")]
async fn update_item(db:web::Data<Database>,item:web::Json<CartItem>) -> HttpResponse {
  let item = db.update_cart_item(item.into_inner());
  match item {
    Ok(item) => HttpResponse::Ok().json(item),
    Err(_) => HttpResponse::InternalServerError().body("Internal error")
  }
}

pub fn init_routes(cfg:&mut web::ServiceConfig){ 
    cfg.service( 
      web::scope("/cart") 
        .service(get_cart)
        .service(add_to_cart)
        .service(remove_from_cart)
        .service(update_item)
     ); 
  }