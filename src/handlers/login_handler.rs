

use actix_web::{web,get,post,delete,put,HttpResponse};
// use diesel::result::Error::NotFound; 
use crate::models::customer::{NewCustomer};
// use crate::models::product::{Product,NewProduct, self};
use crate::repository::database::Database;


//productssssssssssssssssssssssssssssssss
//1.register
#[post("/register")]
async fn add_user(db:web::Data<Database>,user:web::Json<NewCustomer>)->HttpResponse{
    let user = db.add_user(user.into_inner());
    match user {
        Ok(user)=>{HttpResponse::Ok().json(user)},
        Err(_)=>HttpResponse::InternalServerError().body("Internal Server Error")
    }
}

pub fn init_routes(cfg:&mut web::ServiceConfig){ 
  cfg.service( 
    web::scope("/user") 
      .service(add_user)
   ); 
}