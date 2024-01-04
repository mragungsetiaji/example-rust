extern crate serde_json;

use super::model::Tag;
use super::response;
use crate::error::AppError;
use crate::AppState;

use actix_web::{error::Error as ActixWebErr, web, HttpResponse};
use anyhow::Result;


pub async fn index(state: web::Data<AppState>) -> Result<HttpResponse, ActixWebErr> {
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");
    
    // This line runs the Tag::list(&conn) function in a separate thread 
    // to avoid blocking the current thread. web::block is used to offload 
    // blocking I/O or CPU-bound operations to the Actix actor thread pool. 
    // If there's an error, it maps that error to a closure.
    let list = Tag::list(&conn)
        .map_err::<AppError, _>(|_| AppError::HogeError("test".to_string()).into())?;
    
    let res = response::TagsResponse::from(list);
    // If everything goes well, it returns an HttpResponse 
    // with a status of Ok (200) and the body as the JSON representation of list.
    Ok(HttpResponse::Ok().json(res))
}

