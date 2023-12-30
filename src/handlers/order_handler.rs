use actix_web::{web,get,HttpResponse};
use crate::{repository::database::Database,models::orderitem::OrderItem};

#[get("/getorders/{user_id}")]
async fn get_orders(db:web::Data<Database>,user_id:web::Path<i32>) -> HttpResponse {
    let orders = db.get_orders(user_id.into_inner());
    HttpResponse::Ok().json(orders)
}

#[get("/getorder/{order_id}")]
async fn get_order(db:web::Data<Database>,order_id:web::Path<i32>) -> HttpResponse {
    let orderitems = db.get_order_by_id(order_id.into_inner());
    HttpResponse::Ok().json(orderitems)
}

pub fn init_routes(cfg:&mut web::ServiceConfig){ 
    cfg.service( 
      web::scope("/order") 
        .service(get_orders)
        .service(get_order)
     ); 
  }