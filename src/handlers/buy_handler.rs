use actix_web::{web,get,post,HttpResponse,delete,put};
use crate::{repository::database::Database, models::cartitem::CartItem};

#[post("/buy/{userid}/{productid}")]
async fn direct_buy(db:web::Data<Database>,path:web::Path<(i32,i32)>) -> HttpResponse {
    let order = db.direct_buy(path.0,path.1);
    match order {
        Ok(order) => HttpResponse::Ok().json(order),
        Err(_) => HttpResponse::NotFound().body("not found")
    }
}

#[post("/buycartitem")]
async fn buy_cart_item(db:web::Data<Database>,item:web::Json<CartItem>) -> HttpResponse {
    let item = db.buy_cart_item(item.into_inner());
    match item {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(_) => HttpResponse::InternalServerError().body("Internal server eroor")
    }
}

#[post("/checkout/{user_id}")]
async fn checkout(db:web::Data<Database>,user_id:web::Path<i32>) -> HttpResponse {
    let order = db.checkout_cart(user_id.into_inner());
    match order {
        Ok(order) => HttpResponse::Ok().json(order),
        Err(_) => HttpResponse::InternalServerError().body("internal error")
    }
}

pub fn init_routes(cfg:&mut web::ServiceConfig){ 
    cfg.service( 
      web::scope("/buy") 
        .service(direct_buy)
        .service(buy_cart_item)
        .service(checkout)
     ); 
  }