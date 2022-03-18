use actix_web::{web, Result, Responder};
use crate::models::test::{TestObject};

// pub async fn test() -> Result<HttpResponse, Error> {
//     Ok(HttpResponse::Ok().finish())
// }

pub async fn test() -> Result<impl Responder> {
    let response = TestObject {
        name: "test".to_string(),
    };

    Ok(web::Json(response))
}
