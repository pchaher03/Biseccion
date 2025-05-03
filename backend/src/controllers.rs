use actix_web::{post, web, Responder, HttpResponse};
use crate::models::BisectionRequest;
use crate::services::bisection_method;

#[post("/solve")]
pub async fn solve_polynomial(req: web::Json<BisectionRequest>) -> impl Responder {
    match bisection_method(&req.polynomial, req.a, req.b) {
        Some(root) => HttpResponse::Ok().json(root),
        None => HttpResponse::BadRequest().body("Could not find root within given range"),
    }
}
