// use std::path;

use actix_web::{web,get,post,delete,put,HttpResponse};
// use diesel::result::Error::NotFound; 
use crate::models::product::{Product,NewProduct};
use crate::repository::database::Database;


//productssssssssssssssssssssssssssssssss
//1.add the product
#[post("/addproduct")]
async fn create_product(db:web::Data<Database>,product:web::Json<NewProduct>)->HttpResponse{
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
#[get("/getproduct/{id}")]
async fn get_product(db:web::Data<Database>,path:web::Path<i32>) -> HttpResponse {
    let product = db.get_product(path.into_inner());
    match product {
        Some(product) =>HttpResponse::Ok().json(product),
        None => HttpResponse::NotFound().body("not found")
    }
}
//4.update the product 
#[put("/updateproduct")]
async fn update_product(db:web::Data<Database>,product:web::Json<Product>)->HttpResponse{ 
  let product = db.update_product(product.into_inner()); 
  match product { 
    Ok(product)=>HttpResponse::Ok().json(product), 
    Err(_)=>HttpResponse::InternalServerError().body("Internal Server Error") 
  } 
}

//5.deletetheproduct
#[delete("/product/{id}")] 
async fn delete_product(db:web::Data<Database>,path:web::Path<i32>)->HttpResponse{ 
  let product = db.delete_product(path.into_inner()); 
  match product { 
    Ok(product)=>HttpResponse::Ok().json(product), 
    Err(_)=>HttpResponse::InternalServerError().body("Internal Server Error") 
  } 
} 

//6.get all users
#[get("/getusers")]
async fn get_users(db:web::Data<Database>) -> HttpResponse {
  let users = db.get_users();
  HttpResponse::Ok().json(users)
}

//7.get user by id
#[get("/getuser/{id}")]
async fn get_user(db:web::Data<Database>,path:web::Path<i32>) -> HttpResponse {
    let product = db.get_user(path.into_inner());
    match product {
        Some(product) =>HttpResponse::Ok().json(product),
        None => HttpResponse::NotFound().body("not found")
    }
}

pub fn init_routes(cfg:&mut web::ServiceConfig){ 
    cfg.service( 
      web::scope("/admin") 
        .service(create_product)
        .service(get_products)
        .service(get_product)
        .service(update_product)
        .service(delete_product)
        .service(get_users)
        .service(get_user)
     ); 
  }
