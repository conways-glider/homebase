use crate::models::test::{TestBody, TestObject};
use actix_web::{web, Responder, Result};

// pub async fn test() -> Result<HttpResponse, Error> {
//     Ok(HttpResponse::Ok().finish())
// }

pub async fn index() -> Result<impl Responder> {
    let response = TestObject {
        name: "test".to_string(),
    };

    Ok(web::Json(response))
}

pub async fn post(info: web::Json<TestBody>) -> Result<impl Responder> {
    let info = info.into_inner();
    let response = TestObject { name: info.name };

    Ok(web::Json(response))
}
