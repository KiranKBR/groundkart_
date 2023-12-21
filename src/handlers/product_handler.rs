use std::path;

use actix_web::{web,get,post,delete,put,HttpResponse};
use diesel::result::Error::NotFound; 
use crate::models::product::{Product,NewProduct, self};
use crate::repository::database::Database;
use env_logger::Env;

//productssssssssssssssssssssssssssssssss
//1.add the product
#[post("/addproduct")]
async fn create_product(db:web::Data<Database>,product:web::Json<NewProduct>)->HttpResponse{
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let product = db.create_product(product.into_inner());
    match product {
        Ok(product)=>{HttpResponse::Ok().json(product)},
        Err(_)=>HttpResponse::InternalServerError().body("Internal Server Error")
    }
}
//2.getproducts
#[get("/getproducts")]
async fn get_products(db:web::Data<Database>) -> HttpResponse {
    let products = db.get_products();
    HttpResponse::Ok().json(products)
}

//3.get product by id
#[get("getproduct/{id}")]
async fn get_product(db:web::Data<Database>,path:web::Path<i32>) -> HttpResponse {
    let product = db.get_product(path.into_inner());
    match product {
        Some(product) =>HttpResponse::Ok().json(product),
        None => HttpResponse::NotFound().body("not found")
    }
}

pub fn init_routes(cfg:&mut web::ServiceConfig){ 
    cfg.service( 
      web::scope("/api") 
        .service(create_product)
        .service(get_products)
        .service(get_product)
     ); 
  }