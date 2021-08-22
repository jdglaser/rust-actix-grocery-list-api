use actix_web::{get, post, Responder, HttpResponse, web, http};
use crate::user::{User, UserTemplate};
use crate::state::AppState;

#[post("/login/")]
async fn login(state: web::Data<AppState>, 
               user_template: web::Json<UserTemplate>) -> impl Responder {
    let result = state.user_service.login(user_template.into_inner()).await;

    match result {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(error) => Err(error)
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .service(login)
    );
}