use crate::{
    db::{DbPool, query_valid_did_doc},
    error::AppError,
};
use actix_web::{
    HttpResponse,
    web::{Data, Path, block},
};

pub async fn query_did_doc(path: Path<String>, pool: Data<DbPool>) -> HttpResponse {
    let did = path.into_inner();
    let mut conn = pool.get().unwrap();
    match block(move || query_valid_did_doc(&mut conn, did))
        .await
        .map_err(|e| AppError::RunTimeError(e.to_string()))
    {
        Ok(res) => match res {
            Ok(doc) => HttpResponse::Ok().json(doc),
            Err(err) => HttpResponse::from_error(err),
        },
        Err(err) => HttpResponse::from_error(err),
    }
}
