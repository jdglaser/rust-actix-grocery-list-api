use actix_web::{post, Responder, HttpResponse, web};
use crate::user::UserTemplate;
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

#[post("/register/")]
async fn register(state: web::Data<AppState>,
                  user_template: web::Json<UserTemplate>) -> impl Responder {
    let result = state.user_service.register_user(user_template.into_inner()).await;

    info!("Here!");

    match result {
        Ok(token) => Ok(HttpResponse::Ok().json(token)),
        Err(error) => Err(error) 
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .service(login)
            .service(register)
    );
}