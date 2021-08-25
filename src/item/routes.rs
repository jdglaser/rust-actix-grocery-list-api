use actix_web::{get, post, Responder, HttpResponse, web, http};

use crate::state;

use crate::item::{Item, ItemTemplate};
use crate::auth::AuthorizationService;
use crate::util::errors::CustomError;


#[get("/{id}")]
async fn get_item(_: AuthorizationService, state: web::Data<state::AppState>, id: web::Path<i32>) -> impl Responder {
    let result = Item::get_item(&state.database_pool, *id).await;

    match result {
        Ok(item) => Ok(HttpResponse::Ok().json(item)),
        Err(_) => Err(CustomError(
            format!("Item {} not found", id),
            http::StatusCode::NOT_FOUND
        ))
    }
}

#[get("")]
async fn get_items(_: AuthorizationService, state: web::Data<state::AppState>) -> impl Responder {
    let result = Item::get_items(&state.database_pool).await;

    match result {
        Ok(items) => Ok(HttpResponse::Ok().json(items)),
        Err(error) => {
            error!("Error: {}", error.to_string());
            Err(CustomError("Problem getting items from database", http::StatusCode::BAD_REQUEST))
        }
    }
}

#[post("")]
async fn create_item(_: AuthorizationService,
                     state: web::Data<state::AppState>, 
                     new_item: web::Json<ItemTemplate>) -> impl Responder {
    let created_item = Item::create_item(&state.database_pool, new_item.into_inner()).await;

    match created_item {
        Ok(item) => Ok(HttpResponse::Ok().json(item)),
        Err(err) => Err(CustomError(format!("Error creating item: {}", err), http::StatusCode::BAD_REQUEST))
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/item")
            .service(get_item)
            .service(get_items)
            .service(create_item)
    );
}

#[cfg(test)]
mod tests {
    use actix_web::{test, dev};
    use sqlx::SqlitePool;
    use crate::item::{ItemTemplate};
    use crate::util::test_util;

    async fn create_test_item(db: &SqlitePool) -> dev::ServiceResponse {
        let new_item = ItemTemplate {
            name: "foo".to_string(),
            category: "bar".to_string()
        };

        let req = test::TestRequest::post()
            .uri("/item")
            .set_json(&new_item);

        test_util::make_request(req, &db).await
    }

    #[actix_rt::test]
    async fn test_create_item() {
        let db = test_util::setup_test_db().await;

        let resp = create_test_item(&db).await;
        assert!(resp.status().is_success());

        let body = test_util::get_resp_body(resp).await;

        assert_eq!("foo", body["name"]);
        assert_eq!("bar", body["category"]);
    }

    #[actix_rt::test]
    async fn test_get_items() {
        let db = test_util::setup_test_db().await;

        create_test_item(&db).await;
        let req = test::TestRequest::get()
            .uri(&format!("/item/{}", 1));

        let resp = test_util::make_request(req, &db).await;
        assert!(resp.status().is_success());

        let body = test_util::get_resp_body(resp).await;
        assert_eq!("foo", body["name"]);
        assert_eq!("bar", body["category"]);

        let req = test::TestRequest::get()
            .uri("/item");

        let resp = test_util::make_request(req, &db).await;
        let body = test_util::get_resp_body(resp).await;

        let arr = body.as_array().unwrap();
        assert_eq!(1,arr.len());
    }
}

