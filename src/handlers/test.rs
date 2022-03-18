use crate::models::test::TestObject;
use actix_web::{web, Responder, Result};

// pub async fn test() -> Result<HttpResponse, Error> {
//     Ok(HttpResponse::Ok().finish())
// }

pub async fn test() -> Result<impl Responder> {
    let response = TestObject {
        name: "test".to_string(),
    };

    Ok(web::Json(response))
}
