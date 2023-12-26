use super::model::Tag;
use crate::utils::db::DbPool;
use actix_web::{get, web, Responder};

#[get("")]
pub async fn index(pool: web::Data<DbPool>) -> Result<HttpResponse, HttpResponse> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    
    // This line runs the Tag::list(&conn) function in a separate thread 
    // to avoid blocking the current thread. web::block is used to offload 
    // blocking I/O or CPU-bound operations to the Actix actor thread pool. 
    // If there's an error, it maps that error to a closure.
    let tags = web::block(move || Tag::list(&conn)).await.map_err(|e| {

        // Inside the error handling closure, it prints the error to the standard 
        // error and returns an HttpResponse with a status of InternalServerError (500) 
        // and the error message as the body. The ? operator at the end will return 
        // early if an error occurs.
        eprintln!("{}", e);
        HttpResponse::InternalServerError().json(e.to_string())
    })?;
    
    // If everything goes well, it returns an HttpResponse 
    // with a status of Ok (200) and the body as the JSON representation of list.
    Ok(HttpResponse::Ok().json(list))
}

